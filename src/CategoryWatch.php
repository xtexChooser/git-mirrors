<?php

/**
 * CategoryWatch extension
 * - Extends watchlist functionality to include notification about membership
 *   changes of watched categories
 *
 * Copyright (C) 2008  Aran Dunkley
 * Copyright (C) 2017  Sean Chen
 * Copyright (C) 2017  Mark A. Hershberger
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * See https://www.mediawiki.org/Extension:CategoryWatch
 *     for installation and usage details
 * See http://www.organicdesign.co.nz/Extension_talk:CategoryWatch
 *     for development notes and disucssion
 *
 * @file
 * @ingroup Extensions
 * @author Aran Dunkley [http://www.organicdesign.co.nz/nad User:Nad]
 * @copyright © 2008 Aran Dunkley
 * @licence GNU General Public Licence 2.0 or later
 */

namespace CategoryWatch;

class CategoryWatch {
	public $before = [];
	public $after = [];

	protected $count = 0;
	protected $allParents = [];

	protected $wikiPage;
	protected $editor;
	protected $content;
	protected $summary;
	protected $minorEdit;
	protected $flags;

	/**
	 * Construction
	 * @param WikiPage $wikiPage the page
	 * @param User $user who is modifying
	 * @param Content $content the new article content
	 * @param string $summary the article summary (comment)
	 * @param bool $isMinor minor flag
	 * @param int $flags see WikiPage::doEditContent documentation for flags' definition
	 */
	public function __construct(
		WikiPage $wikiPage, User $user, Content $content, $summary, $isMinor, $flags
	) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		$this->wikiPage;
		$this->editor;
		$this->content;
		$this->summary;
		$this->minorEdit;
		$this->flags;

		$this->before = $this->wikiPage->getTitle()->getParentCategories();
		$this->doAutoCat();
	}

	/**
	 * Notify all category watchers
	 *
	 * @param Revision $revision that was created
	 * @param int $baseRevId base revision
	 */
	public function notifyCategoryWatchers(
		Revision $revision, $baseRevId
	) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		# Get cats after update
		$this->after = $this->wikiPage->getTitle()->getParentCategories();

		# Get list of added and removed cats
		$add = array_diff( $this->after, $this->before );
		$sub = array_diff( $this->before, $this->after );
		wfDebugLog( 'CategoryWatch', 'Categories after page saved' );
		wfDebugLog( 'CategoryWatch', join( ', ', $this->after ) );
		wfDebugLog( 'CategoryWatch', 'Categories added' );
		wfDebugLog( 'CategoryWatch', join( ', ', $add ) );
		wfDebugLog( 'CategoryWatch', 'Categories removed' );
		wfDebugLog( 'CategoryWatch', join( ', ', $sub ) );

		# Notify watchers of each cat about the addition or removal of this article
		if ( count( $add ) > 0 || count( $sub ) > 0 ) {
			$page     = $article->getTitle();
			$pagename = $page->getPrefixedText();
			$pageurl  = $page->getFullUrl();
			$page     = "$pagename ($pageurl)";

			if ( count( $add ) == 1 && count( $sub ) == 1 ) {
				$this->notifyMove( $sub[0], $add[0] );
			} else {
				$this->notifyAdd( $add );

				foreach ( $sub as $cat ) {
					$title   = Title::newFromText( $cat, NS_CATEGORY );
					$message = wfMessage(
						'categorywatch-catsub', $page,
						$this->friendlyCat( $cat )
					)->text();
					$this->notifyWatchers(
						$title, $user, $message, $summary, $medit, $pageurl
					);
				}
			}
		}

		if ( $this->shouldNotifyParentWatchers() ) {
			$this->notifyParentWatchers();
		}
	}

	/**
	 * Should watchers of parent categories be notified?
	 * @return bool
	 */
	protected function shouldNotifyParentWatchers() {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		global $wgCategoryWatchNotifyParentWatchers;
		return $wgCategoryWatchNotifyParentWatchers;
	}

	/**
	 * Should the editor be notified of his own edits?
	 * @return bool
	 */
	protected function shouldNotifyEditor() {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		global $wgCategoryWatchNotifyEditor;
		return $wgCategoryWatchNotifyEditor;
	}

	/**
	 * Should CategoryWatch use the user's real name in email?
	 * @return bool
	 */
	protected function useRealName() {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		global $wgCategoryWatchNoRealName;
		return !$wgCategoryWatchNoRealName;
	}

	/**
	 * Return "Category:Cat (URL)" from "Cat"
	 * @param string $cat name of category
	 * @return string
	 */
	protected function friendlyCat( $cat ) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		$cat     = Title::newFromText( $cat, NS_CATEGORY );
		$catname = $cat->getPrefixedText();
		$caturl  = $cat->getFullUrl();
		return "$catname ($caturl)";
	}

	/**
	 * Notify any watchers
	 * @param Title $title of article
	 * @param User $editor of article
	 * @param string $message for user
	 * @param string $summary editor gave
	 * @param bool $medit true if minor
	 * @param string $pageurl of page
	 */
	protected function notifyWatchers(
		$title, $editor, $message, $summary, $medit, $pageurl
	) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		global $wgLang, $wgNoReplyAddress,
			$wgEnotifRevealEditorAddress, $wgEnotifUseRealName, $wgPasswordSender,
			$wgEnotifFromEditor, $wgPasswordSenderName;

		# Get list of users watching this category
		$dbr = wfGetDB( DB_SLAVE );
		$conds = [
			'wl_title' => $title->getDBkey(), 'wl_namespace' => $title->getNamespace()
		];
		if ( !$this->shouldNotifyEditor() ) {
			$conds[] = 'wl_user <> ' . intval( $editor->getId() );
		}
		$res = $dbr->select( 'watchlist', [ 'wl_user' ], $conds, __METHOD__ );

		# Wrap message with common body and send to each watcher
		$page = $title->getPrefixedText();
		$adminAddress   = new MailAddress(
			$wgPasswordSender,
			isset( $wgPasswordSenderName )
			? $wgPasswordSenderName
			: 'WikiAdmin'
		);
		$editorAddress  = new MailAddress( $editor );
		$summary        = $summary
						? $summary
						: ' - ';
		$medit          = $medit
						? wfMessage( 'minoredit' )->text()
						: '';
		$row            = $dbr->fetchRow( $res );
		while ( $row ) {
			$watchingUser   = User::newFromId( $row[0] );
			$timecorrection = $watchingUser->getOption( 'timecorrection' );
			$editdate       = $wgLang->timeanddate(
				wfTimestampNow(), true, false, $timecorrection
			);

			if (
				$watchingUser->getOption( 'enotifwatchlistpages' )
				&& $watchingUser->isEmailConfirmed()
			) {
				$to      = new MailAddress( $watchingUser );
				$subject = wfMessage( 'categorywatch-emailsubject', $page )->text();
				$body    = wfMessage( 'enotif_body' )->inContentLanguage()->text();

				# Reveal the page editor's address as REPLY-TO address only if
				# the user has not opted-out and the option is enabled at the
				# global configuration level.
				$name = $wgEnotifUseRealName
					  ? $watchingUser->getRealName()
					  : $watchingUser->getName();
				if ( $wgEnotifRevealEditorAddress
					 && ( $editor->getEmail() != '' )
					 && $editor->getOption( 'enotifrevealaddr' )
				) {
					if ( $wgEnotifFromEditor ) {
						$from = $editorAddress;
					} else {
						$from = $adminAddress;
						$replyto = $editorAddress;
					}
				} else {
					$from = $adminAddress;
					$replyto = new MailAddress( $wgNoReplyAddress );
				}

				# Define keys for body message
				$userPage = $editor->getUserPage();
				$keys = [
					'$WATCHINGUSERNAME' => $name,
					'$NEWPAGE'          => $message,
					'$PAGETITLE'        => $page,
					'$PAGEEDITDATE'     => $editdate,
					'$CHANGEDORCREATED' => wfMessage( 'changed' )
					->inContentLanguage()->text(),
					'$PAGETITLE_URL'    => $title->getFullUrl(),
					'$PAGEEDITOR_WIKI'  => $userPage->getFullUrl(),
					'$PAGESUMMARY'      => $summary,
					'$PAGEMINOREDIT'    => $medit,
					'$OLDID'            => ''
				];
				if ( $editor->isIP( $name ) ) {
					$utext = wfMessage(
						'enotif_anon_editor', $name
					)->inContentLanguage()->text();
					$subject = str_replace( '$PAGEEDITOR', $utext, $subject );
					$keys['$PAGEEDITOR'] = $utext;
					$keys['$PAGEEDITOR_EMAIL'] = wfMmessage(
						'noemailtitle'
					)->inContentLanguage()->text();
				} else {
					$subject = str_replace( '$PAGEEDITOR', $name, $subject );
					$keys['$PAGEEDITOR'] = $name;
					$emailPage = SpecialPage::getSafeTitleFor( 'Emailuser', $name );
					$keys['$PAGEEDITOR_EMAIL'] = $emailPage->getFullUrl();
				}
				$keys['$PAGESUMMARY'] = $summary;

				# Replace keys, wrap text and send
				$body = strtr( $body, $keys );
				$body = wordwrap( $body, 72 );
				$options = [];
				$options['replyTo'] = $replyto;
				UserMailer::send( $to, $from, $subject, $body, $options );
			}
		}

		$dbr->freeResult( $res );
	}

	/**
	 * Notify the watchers of parent categories
	 */
	protected function notifyParentWatchers() {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		$this->allparents = $this->wikiPage->getTitle()->getParentCategoryTree();
		$page = $this->wikiPage->getTitle();
		$pageUrl = $page->getFullUrl();
		foreach ( (array)$this->allparents as $cat ) {
			$title   = Title::newFromText( $cat, NS_CATEGORY );
			$message = wfMessage(
				'categorywatch-catchange', $page,
				$this->friendlyCat( $cat )
			);
			$this->notifyWatchers(
				$title, $user, $message, $summary, $medit, $pageurl
			);
		}
	}

	/**
	 * Handle autocat option
	 */
	protected function doAutoCat() {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		global $wgCategoryWatchUseAutoCat;
		if ( $wgCategoryWatchUseAutoCat ) {
			$dbr = wfGetDB( DB_SLAVE );

			# Find all users not watching the autocat
			$like = '%' . str_replace(
				' ', '_', trim( wfMessage( 'categorywatch-autocat', '' )->text() )
			) . '%';
			$res = $dbr->select( [ 'user', 'watchlist' ], 'user_id',
								 'wl_user IS NULL', __METHOD__, [],
								 [ 'watchlist' => [ 'LEFT JOIN',
													[
														'user_id=wl_user',
														'wl_tile', $dbr->buildLike( $like )
													] ] ] );

			# Insert an entry into watchlist for each
			$row = $dbr->fetchRow( $res );
			while ( $row ) {
				$user = User::newFromId( $row[0] );
				$name = $user->getName();
				$wl_title = str_replace(
					' ', '_', wfMessage( 'categorywatch-autocat', $name )->text()
				);
				$dbr->insert(
					'watchlist',
					[
						'wl_user' => $row[0], 'wl_namespace' => NS_CATEGORY,
						'wl_title' => $wl_title
					]
				);
				$row = $dbr->fetchRow( $res );
			}
			$dbr->freeResult( $res );
		}
	}

	/**
	 * Send a notification that the page's categorization has moved.
	 * @param string $from Category moving from
	 * @param string $to Category moving to
	 */
	protected function notifyMove( $from, $to ) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		$title   = Title::newFromText( $to, NS_CATEGORY );
		$message = wfMessage(
			'categorywatch-catmovein', $page,
			$this->friendlyCat( $to ),
			$this->friendlyCat( $from )
		)->text();
		$this->notifyWatchers(
			$title, $user, $message, $summary, $medit, $pageurl
		);

		$title   = Title::newFromText( $from, NS_CATEGORY );
		$message = wfMessage(
			'categorywatch-catmoveout', $page,
			$this->friendlyCat( $from ),
			$this->friendlyCat( $to )
		)->text();
		$this->notifyWatchers(
			$title, $user, $message, $summary, $medit, $pageurl
		);
	}

	/**
	 * Send a notification that a page has been added to the category
	 * @param array $add Category being added
	 */
	protected function notifyAdd( $add ) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		foreach ( $add as $cat ) {
			$title   = Title::newFromText( $cat, NS_CATEGORY );
			$message = wfMessage(
				'categorywatch-catadd', $page,
				$this->friendlyCat( $cat )
			)->text();
			$this->notifyWatchers(
				$title, $user, $message, $summary, $medit, $pageurl
			);
		}
	}
}
