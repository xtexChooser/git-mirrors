<?php
if ( !defined( 'MEDIAWIKI' ) ) die( 'Not an entry point.' );
/**
 * CategoryWatch extension
 * - Extends watchlist functionality to include notification about membership changes of watched categories
 *
 * See http://www.mediawiki.org/Extension:CategoryWatch for installation and usage details
 * See http://www.organicdesign.co.nz/Extension_talk:CategoryWatch for development notes and disucssion
 *
 * @file
 * @ingroup Extensions
 * @author Aran Dunkley [http://www.organicdesign.co.nz/nad User:Nad]
 * @copyright © 2008 Aran Dunkley
 * @licence GNU General Public Licence 2.0 or later
 */

# Whether or not to also send notificaton to the person 
# who made the change
# The default value is set in file extension.json
//$wgCategoryWatchNotifyEditor = true;

# Set this to give every user a unique category that 
# they're automatically watching
# - the format of the category name is defined on the 
# "categorywatch-autocat" localisation message
# The default value is set in file extension.json
//$wgCategoryWatchUseAutoCat = false;

# Set this to make the categorisation work by realname 
# instead of username
# The default value is set in file extension.json
//$wgCategoryWatchUseAutoCatRealName = false;

class CategoryWatch {

	/**
	 * The extension function.
	 * It has to be the static function in a class now.
	 */
	public static function wfSetupCategoryWatch() {
		wfDebugLog('CategoryWatch', 'loading extension...');
        	global $wgCategoryWatch;

        	# Instantiate the CategoryWatch singleton now 
                # that the environment is prepared
        	$wgCategoryWatch = new CategoryWatch();
	}

	function __construct() {
		# the constructor will do nothing now.
		# New extension register process will use the file
		# extension.json to set hooks.
	}

	/**
	 * Get a list of categories before article updated
	 * Since MediaWiki version 1.25.x, we have to use static function
	 * for hooks.
	 * the hook has different signatures.
	 */
	public static function onPageContentSave( &$wikiPage, &$user, &$content, &$summary,	$isMinor, $isWatch, $section, &$flags, &$status) {

		global $wgCategoryWatchUseAutoCat, $wgCategoryWatchUseAutoCatRealName, $wgCategoryWatch;

		$wgCategoryWatch->before = array();
		$dbr  = wfGetDB( DB_MASTER);
		$cl   = $dbr->tableName( 'categorylinks' );
		wfDebugLog('CategoryWatch', "tablename = $cl");
		$id   = $wikiPage->getID();
		wfDebugLog('CategoryWatch', "page id=$id");
		$res  = $dbr->select( $cl, 'cl_to', "cl_from = $id", __METHOD__, array( 'ORDER BY' => 'cl_sortkey' ) );
		while ( $row = $dbr->fetchRow( $res ) ) $wgCategoryWatch->before[] = $row[0];
		$dbr->freeResult( $res );
		wfDebugLog('CategoryWatch', 'Categories before page saved');
		wfDebugLog('CategoryWatch', join(', ', $wgCategoryWatch->before));

		# If using the automatically watched category feature, ensure that all users are watching it
		if ( $wgCategoryWatchUseAutoCat ) {
			$dbr = wfGetDB( DB_SLAVE );

			# Find all users not watching the autocat
			$like = str_replace( ' ', '_', trim( wfMessage( 'categorywatch-autocat', '' )->text() ) );
			$utbl = $dbr->tableName( 'user' );
			$wtbl = $dbr->tableName( 'watchlist' );
			$sql = "SELECT user_id FROM $utbl LEFT JOIN $wtbl ON user_id=wl_user AND wl_title LIKE '%$like%' WHERE wl_user IS NULL";
			$res = $dbr->query( $sql );

			# Insert an entry into watchlist for each
			while ( $row = $dbr->fetchRow( $res ) ) {
				$user = User::newFromId( $row[0] );
				$name = $wgCategoryWatchUseAutoCatRealName ? $user->getRealName() : $user->getName();
				$wl_title = str_replace( ' ', '_', wfMessage( 'categorywatch-autocat', $name )->text() );
				$dbr->insert( $wtbl, array( 'wl_user' => $row[0], 'wl_namespace' => NS_CATEGORY, 'wl_title' => $wl_title ) );
			}
			$dbr->freeResult( $res );
		}

		return true;
	}
	
	/**
	 * the proper hook for save page request.
	 * @see http://www.mediawiki.org/wiki/Manual:Hooks/PageContentSaveComplete
	 * @param $article Article edited
	 * @param $user User who edited
	 * @param $content Content New article text
	 * @param $summary string Edit summary
	 * @param $isMinor bool Minor edit or not
	 * @param $isWatch bool Watch this article?
	 * @param $section string Section that was edited
	 * @param $flags int Edit flags
	 * @param $revision Revision that was created
	 * @param $status Status
	 * @return bool true in all cases
	*/
	public static function onPageContentSaveComplete($article, $user, $content, $summary, $isMinor, $isWatch, $section, $flags, $revision, $status, $baseRevId) {

		global $wgCategoryWatch;

		# Get cats after update
		$wgCategoryWatch->after = array();

		$parseTimestamp = $revision->getTimestamp();
		$content = $revision->getContent();
		$title = $article->getTitle();
		$options = $content->getContentHandler()->makeParserOptions('canonical');
		$options->setTimestamp($parseTimestamp);
		$output = $content->getParserOutput( $title, $revision->getId(), $options);
		$wgCategoryWatch->after = array_map('strval', array_keys($output->getCategories()));
		wfDebugLog('CategoryWatch', 'Categories after page saved');
		wfDebugLog('CategoryWatch', join(', ', $wgCategoryWatch->after));

		# Get list of added and removed cats
		$add = array_diff( $wgCategoryWatch->after, $wgCategoryWatch->before );
		$sub = array_diff( $wgCategoryWatch->before, $wgCategoryWatch->after );

		# Notify watchers of each cat about the addition or removal of this article
		if ( count( $add ) > 0 || count( $sub ) > 0 ) {
			$page     = $article->getTitle();
			$pagename = $page->getPrefixedText();
			$pageurl  = $page->getFullUrl();
			$page     = "$pagename ($pageurl)";

			if ( count( $add ) == 1 && count( $sub ) == 1 ) {

				$add = array_shift( $add );
				$sub = array_shift( $sub );

				$title   = Title::newFromText( $add, NS_CATEGORY );
				$message = wfMessage( 'categorywatch-catmovein', $page, $wgCategoryWatch->friendlyCat( $add ), $wgCategoryWatch->friendlyCat( $sub ) )->text();
				$wgCategoryWatch->notifyWatchers( $title, $user, $message, $summary, $medit );

				$title   = Title::newFromText( $sub, NS_CATEGORY );
				$message = wfMessage( 'categorywatch-catmoveout', $page, $wgCategoryWatch->friendlyCat( $sub ), $wgCategoryWatch->friendlyCat( $add ) )->text();
				$wgCategoryWatch->notifyWatchers( $title, $user, $message, $summary, $medit );
			} else {

				foreach ( $add as $cat ) {
					$title   = Title::newFromText( $cat, NS_CATEGORY );
					$message = wfMessage( 'categorywatch-catadd', $page, $wgCategoryWatch->friendlyCat( $cat ) )->text();
					$wgCategoryWatch->notifyWatchers( $title, $user, $message, $summary, $medit );
				}

				foreach ( $sub as $cat ) {
					$title   = Title::newFromText( $cat, NS_CATEGORY );
					$message = wfMessage( 'categorywatch-catsub', $page, $wgCategoryWatch->friendlyCat( $cat ) )->text();
					$wgCategoryWatch->notifyWatchers( $title, $user, $message, $summary, $medit );
				}
			}
		}

		return true;
	}

	/**
	 * Return "Category:Cat (URL)" from "Cat"
	 */
	function friendlyCat( $cat ) {
		$cat     = Title::newFromText( $cat, NS_CATEGORY );
		$catname = $cat->getPrefixedText();
		$caturl  = $cat->getFullUrl();
		return "$catname ($caturl)";
	}

	function notifyWatchers( &$title, &$editor, &$message, &$summary, &$medit ) {
		global $wgLang, $wgEmergencyContact, $wgNoReplyAddress, $wgCategoryWatchNotifyEditor,
			$wgEnotifRevealEditorAddress, $wgEnotifUseRealName, $wgPasswordSender, $wgEnotifFromEditor;

		# Get list of users watching this category
		$dbr = wfGetDB( DB_SLAVE );
		$conds = array( 'wl_title' => $title->getDBkey(), 'wl_namespace' => $title->getNamespace() );
		if ( !$wgCategoryWatchNotifyEditor ) $conds[] = 'wl_user <> ' . intval( $editor->getId() );
		$res = $dbr->select( 'watchlist', array( 'wl_user' ), $conds, __METHOD__ );

		# Wrap message with common body and send to each watcher
		$page           = $title->getPrefixedText();
		# $wgPasswordSenderName was introduced only in MW 1.17
		global $wgPasswordSenderName;
		$adminAddress   = new MailAddress( $wgPasswordSender,
			isset( $wgPasswordSenderName ) ? $wgPasswordSenderName : 'WikiAdmin' );
		$editorAddress  = new MailAddress( $editor );
		$summary        = $summary ? $summary : ' - ';
		$medit          = $medit ? wfMessage( 'minoredit' )->text() : '';
		while ( $row = $dbr->fetchRow( $res ) ) {
			$watchingUser   = User::newFromId( $row[0] );
			$timecorrection = $watchingUser->getOption( 'timecorrection' );
			$editdate       = $wgLang->timeanddate( wfTimestampNow(), true, false, $timecorrection );

			if ( $watchingUser->getOption( 'enotifwatchlistpages' ) && $watchingUser->isEmailConfirmed() ) {
				$to      = new MailAddress( $watchingUser );
				$subject = wfMessage( 'categorywatch-emailsubject', $page )->text();
				$body    = wfMessage( 'enotif_body' )->inContentLanguage()->text();

				# Reveal the page editor's address as REPLY-TO address only if
				# the user has not opted-out and the option is enabled at the
				# global configuration level.
				$name = $wgEnotifUseRealName ? $watchingUser->getRealName() : $watchingUser->getName();
				if ( $wgEnotifRevealEditorAddress
					&& ( $editor->getEmail() != '' )
					&& $editor->getOption( 'enotifrevealaddr' ) ) {
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
				$keys = array(
					'$WATCHINGUSERNAME' => $name,
					'$NEWPAGE'          => $message,
					'$PAGETITLE'        => $page,
					'$PAGEEDITDATE'     => $editdate,
					'$CHANGEDORCREATED' => wfMessage( 'changed' )->inContentLanguage()->text(),
					'$PAGETITLE_URL'    => $title->getFullUrl(),
					'$PAGEEDITOR_WIKI'  => $userPage->getFullUrl(),
					'$PAGESUMMARY'      => $summary,
					'$PAGEMINOREDIT'    => $medit,
					'$OLDID'            => ''
				);
				if ( $editor->isIP( $name ) ) {
					$utext = wfMessage( 'enotif_anon_editor', $name )->inContentLanguage()->text();
					$subject = str_replace( '$PAGEEDITOR', $utext, $subject );
					$keys['$PAGEEDITOR'] = $utext;
					$keys['$PAGEEDITOR_EMAIL'] = wfMmessage( 'noemailtitle' )->inContentLanguage()->text();
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
				UserMailer::send( $to, $from, $subject, $body, $options);
			}
		}

		$dbr->freeResult( $res );
	}

	/**
	 * Needed in some versions to prevent Special:Version from breaking
	 */
	function __toString() { return __CLASS__; }
}

