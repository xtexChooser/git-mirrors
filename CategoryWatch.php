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
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
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

class CategoryWatch {
	// Instance
	protected static $watcher;

	/**
	 * The extension function.
	 * It has to be the static function in a class now.
	 */
	public static function setupCategoryWatch() {
		wfDebugLog( 'CategoryWatch', 'loading extension...' );

		# Instantiate the CategoryWatch singleton now
		# that the environment is prepared
		self::$watcher = new CategoryWatch();
	}

	/**
	 * Get a list of categories before article updated Since MediaWiki
	 * version 1.25.x, we have to use static function for hooks.  the
	 * hook has different signatures.
	 * @param WikiPage $wikiPage the page
	 * @param User $user who is modifying
	 * @param Content $content the new article content
	 * @param string $summary the article summary (comment)
	 * @param bool $isMinor minor flag
	 * @param bool $isWatch watch flag (not used, aka always null)
	 * @param int $section section number (not used, aka always null)
	 * @param int $flags see WikiPage::doEditContent documentation for flags' definition
	 * @param Status $status Status (object)
	 */
	public static function onPageContentSave(
		WikiPage $wikiPage, $user, $content, $summary, $isMinor,
		$isWatch, $section, $flags, $status
	) {
		global $wgCategoryWatchUseAutoCat, $wgCategoryWatchUseAutoCatRealName,
			$wgCategoryWatch;

		self::$watcher->before = [];
		$dbr  = wfGetDB( DB_MASTER );
		$cl   = $dbr->tableName( 'categorylinks' );
		$id   = $wikiPage->getID();
		wfDebugLog( 'CategoryWatch', "tablename = $cl" );
		wfDebugLog( 'CategoryWatch', "page id=$id" );
		$res  = $dbr->select(
			$cl, 'cl_to', "cl_from = $id", __METHOD__,
			[ 'ORDER BY' => 'cl_sortkey' ]
		);
		$row = $dbr->fetchRow( $res );
		while ( $row ) {
			self::$watcher->before[] = $row[0];
			$row = $dbr->fetchRow( $res );
		}
		$dbr->freeResult( $res );
		wfDebugLog( 'CategoryWatch', 'Categories before page saved' );
		wfDebugLog( 'CategoryWatch', join( ', ', self::$watcher->before ) );

		# If using the automatically watched category feature, ensure
		# that all users are watching it
		if ( $wgCategoryWatchUseAutoCat ) {
			$dbr = wfGetDB( DB_SLAVE );

			# Find all users not watching the autocat
			$like = str_replace(
				' ', '_',
				trim( wfMessage( 'categorywatch-autocat', '' )->text() )
			);
			$utbl = $dbr->tableName( 'user' );
			$wtbl = $dbr->tableName( 'watchlist' );
			$sql = "SELECT user_id FROM $utbl LEFT JOIN $wtbl ON "
				 . "user_id=wl_user AND wl_title LIKE '%$like%' "
				 . "WHERE wl_user IS NULL";

			# Insert an entry into watchlist for each
			$row = $dbr->fetchRow( $res );
			while ( $row ) {
				$user = User::newFromId( $row[0] );
				$name = $wgCategoryWatchUseAutoCatRealName
					  ? $user->getRealName()
					  : $user->getName();
				$wl_title = str_replace(
					' ', '_', wfMessage( 'categorywatch-autocat', $name )->text()
				);
				$dbr->insert(
					$wtbl,
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
	 * the proper hook for save page request.
	 * @see https://www.mediawiki.org/wiki/Manual:Hooks/PageContentSaveComplete
	 *
	 * @param WikiPage $wikiPage WikiPage modified
	 * @param User $user who edited
	 * @param Content $content New article text
	 * @param string $summary Edit summary
	 * @param bool $isMinor Minor edit or not
	 * @param bool $isWatch Watch this article?
	 * @param string $section Section that was edited
	 * @param int $flags Edit flags
	 * @param Revision $revision that was created
	 * @param Status $status of activities
	 * @param int $baseRevId base revision
	 */
	public static function onPageContentSaveComplete(
		WikiPage $wikiPage, $user, $content, $summary, $isMinor, $isWatch, $section,
		$flags, $revision, $status, $baseRevId
	) {
		# Get cats after update
		self::$watcher->after = [];

		$parseTimestamp = $revision->getTimestamp();
		$content = $revision->getContent();
		$title = $wikiPage->getTitle();
		$options = $content->getContentHandler()->makeParserOptions( 'canonical' );
		$options->setTimestamp( $parseTimestamp );
		$output = $content->getParserOutput( $title, $revision->getId(), $options );
		self::$watcher->after = array_map(
			'strval', array_keys( $output->getCategories() )
		);
		wfDebugLog( 'CategoryWatch', 'Categories after page saved' );
		wfDebugLog( 'CategoryWatch', join( ', ', self::$watcher->after ) );

		# Get list of added and removed cats
		$add = array_diff( self::$watcher->after, self::$watcher->before );
		$sub = array_diff( self::$watcher->before, self::$watcher->after );

		# Notify watchers of each cat about the addition or removal of this article
		if ( count( $add ) > 0 || count( $sub ) > 0 ) {
			$page = $wikiPage->getTitle();
			$pagename = $page->getPrefixedText();
			$pageurl  = $page->getFullUrl();
			$page     = "$pagename ($pageurl)";

			if ( count( $add ) == 1 && count( $sub ) == 1 ) {
				$add = array_shift( $add );
				$sub = array_shift( $sub );

				$title   = Title::newFromText( $add, NS_CATEGORY );
				$message = wfMessage(
					'categorywatch-catmovein', $page,
					self::$watcher->friendlyCat( $add ),
					self::$watcher->friendlyCat( $sub )
				)->text();
				self::$watcher->notifyWatchers(
					$title, $user, $message, $summary, $medit, $pageurl
				);

				$title   = Title::newFromText( $sub, NS_CATEGORY );
				$message = wfMessage(
					'categorywatch-catmoveout', $page,
					self::$watcher->friendlyCat( $sub ),
					self::$watcher->friendlyCat( $add )
				)->text();
				self::$watcher->notifyWatchers(
					$title, $user, $message, $summary, $medit, $pageurl
				);
			} else {

				foreach ( $add as $cat ) {
					$title   = Title::newFromText( $cat, NS_CATEGORY );
					$message = wfMessage(
						'categorywatch-catadd', $page,
						self::$watcher->friendlyCat( $cat )
					)->text();
					self::$watcher->notifyWatchers(
						$title, $user, $message, $summary, $medit, $pageurl
					);
				}

				foreach ( $sub as $cat ) {
					$title   = Title::newFromText( $cat, NS_CATEGORY );
					$message = wfMessage(
						'categorywatch-catsub', $page,
						self::$watcher->friendlyCat( $cat )
					)->text();
					self::$watcher->notifyWatchers(
						$title, $user, $message, $summary, $medit, $pageurl
					);
				}
			}
		}

		global $wgCategoryWatchNotifyParentWatchers;
		if ( $wgCategoryWatchNotifyParentWatchers ) {
			self::notifyParentWatchers();
		}
	}

	/**
	 * Notify the watchers of parent categories
	 */
	protected static function notifyParentWatchers() {
		self::$watcher->allparents = [];
		self::$watcher->i = 0;
		self::$watcher->findCategoryParents( self::$watcher->after );
		## For each active parent category, send the mail
		if ( self::$watcher->allparents ) {
			$page     = $article->getTitle();
			$pageurl  = $page->getFullUrl();
			foreach ( self::$watcher->allparents as $cat ) {
				$title   = Title::newFromText( $cat, NS_CATEGORY );
				$message = wfMessage(
					'categorywatch-catchange', $page,
					self::$watcher->friendlyCat( $cat )
				);
				self::$watcher->notifyWatchers(
					$title, $user, $message, $summary, $medit, $pageurl
				);
			}
		}
	}

	/**
	 * Recursively find all parents of the given categories
	 *
	 * @param array $catarray the categories
	 */
	protected function findCategoryParents( array $catarray ) {
		$this->i++;
		if ( $this->i == 200 ) {
			return;
		}

		if ( $catarray ) {
			foreach ( $catarray as $catname ) {
				self::$watcher->allparents[] = $catname;
				$id = self::$watcher->getCategoryArticleId( $catname );
				if ( is_numeric( $id ) ) {
					$parentCat = self::$watcher->getParentCategories( $id );
					if ( $parentCat ) {
						self::$watcher->allparents[] = $parentCat;
						self::$watcher->findCategoryParents( [ $parentCat ] );
					}
				}
			}
			self::$watcher->allparents = array_unique( self::$watcher->allparents );
		}
	}

	/**
	 * Return the parent categories
	 * @param int $id Category Article id
	 * @return parents
	 */
	protected function getParentCategories( $id ) {
		$dbr  = wfGetDB( DB_SLAVE );
		$cl   = $dbr->tableName( 'categorylinks' );
		$res  = $dbr->select(
			$cl, 'cl_to', "cl_from = $id", __METHOD__,
			[ 'ORDER BY' => 'cl_sortkey' ]
		);
		$row = $dbr->fetchRow( $res );
		$dbr->freeResult( $res );
		if ( empty( $row[0] ) ) {
			return false;
		}
		return $row[0];
	}

	/**
	 * Load page ID of one category
	 *
	 * @param string $catname name of category
	 * @return int
	 */
	protected function getCategoryArticleId( $catname ) {
		$dbr = wfGetDB( DB_SLAVE );
		$cl  = $dbr->tableName( 'page' );
		$res = $dbr->select( $cl, 'page_id', "page_title = '$catname'", __METHOD__ );
		$row = $dbr->fetchRow( $res );
		$dbr->freeResult( $res );
		return $row[0];
	}

	/**
	 * Return "Category:Cat (URL)" from "Cat"
	 * @param string $cat name of category
	 * @return string
	 */
	protected function friendlyCat( $cat ) {
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
	function notifyWatchers( $title, $editor, $message, $summary, $medit, $pageurl ) {
		global $wgLang, $wgNoReplyAddress, $wgCategoryWatchNotifyEditor,
			$wgEnotifRevealEditorAddress, $wgEnotifUseRealName, $wgPasswordSender,
			$wgEnotifFromEditor, $wgPasswordSenderName;

		# Get list of users watching this category
		$dbr = wfGetDB( DB_SLAVE );
		$conds = [
			'wl_title' => $title->getDBkey(), 'wl_namespace' => $title->getNamespace()
		];
		if ( !$wgCategoryWatchNotifyEditor ) {
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
				if ( $wgCategoryWatchNoRealName ) {
					$name = $watchingUser->getName();
				}
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
				# body message is defined in page MediaWiki:Enotif_body
				# set both $PAGEINTRO and $HELPPAGE to empty string for now.
				$userPage = $editor->getUserPage();
				$keys = [
					'$WATCHINGUSERNAME' => $name,
					'$PAGEINTRO'        => '',
					'$NEWPAGE'          => $message,
					'$PAGETITLE'        => $page,
					'$PAGEEDITDATE'     => $editdate,
					'$CHANGEDORCREATED' => wfMessage( 'changed' )
					->inContentLanguage()->text(),
					'$PAGETITLE_URL'    => $title->getFullUrl(),
					'$PAGEEDITOR_WIKI'  => $userPage->getFullUrl(),
					'$PAGESUMMARY'      => $summary,
					'$PAGEMINOREDIT'    => $medit,
					'$HELPPAGE'         => '',
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
}
