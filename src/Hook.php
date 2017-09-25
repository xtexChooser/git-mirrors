<?php
/**
 * Hooks for CategoryWatch extension
 *
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
 */
namespace CategoryWatch;

use Category;
use Content;
use EchoDiscussionParser;
use EchoEvent;
use MediaWiki\MediaWikiServices;
use WatchedItemStore;
use Status;
use Title;
use User;
use WikiPage;

class Hook {
	// Instance
	protected static $watcher;

	/**
	 * Explain bundling
	 *
	 * @param Event $event to bundle
	 * @param string &$bundleString to use
	 */
	public static function onEchoGetBundleRules( EchoEvent $event, &$bundleString ) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		switch ( $event->getType() ) {
		case 'categorywatch-add':
		case 'categorywatch-remove':
			$bundleString = 'categorywatch';
			break;
		}
	}

	/**
	 * Define the CategoryWatch notifications
	 *
	 * @param array &$notifications assoc array of notification types
	 * @param array &$notificationCategories assoc array describing
	 *        categories
	 * @param array &$icons assoc array of icons we define
	 */
	public static function onBeforeCreateEchoEvent(
		array &$notifications, array &$notificationCategories, array &$icons
	) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		$icons['categorywatch']['path'] = 'CategoryWatch/assets/catwatch.svg';

		$notifications['categorywatch-add'] = [
			'bundle' => [
				'web' => true,
				'email' => true,
				'expandable' => true,
			],
			'title-message' => 'categorywatch-add-title',
			'category' => 'categorywatch',
			'group' => 'neutral',
			'user-locators' => [ 'CategoryWatch\\Hook::userLocater' ],
			'user-filters' => [ 'CategoryWatch\\Hook::userFilter' ],
			'presentation-model' => 'CategoryWatch\\EchoEventPresentationModel',
		];

		$notifications['categorywatch-remove'] = [
			'bundle' => [
				'web' => true,
				'email' => true,
				'expandable' => true,
			],
			'title-message' => 'categorywatch-remove-title',
			'category' => 'categorywatch',
			'group' => 'neutral',
			'user-locators' => [ 'CategoryWatch\\Hook::userLocater' ],
			'user-filters' => [ 'CategoryWatch\\Hook::userFilter' ],
			'presentation-model' => 'CategoryWatch\\EchoEventPresentationModel',
		];

		$notificationCategories['categorywatch'] = [
			'priority' => 2,
			'tooltip' => 'echo-pref-tooltip-categorywatch'
		];
	}

	/**
	 * Internal compatibility function
	 * @return WatchedItemStore
	 */
	protected static function getWatchedItemStore() {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		if ( method_exists( 'WatchedItemStore', 'getDefaultInstance' ) ) {
			return WatchedItemStore::getDefaultInstance();
		} else {
			return MediaWikiServices::getInstance()->getWatchedItemStore();
		}
	}

	/**
	 * Hook for page being added to a category.
	 *
	 * @param Category $cat that page is being add to
	 * @param WikiPage $page that is being added
	 */
	public static function onCategoryAfterPageAdded(
		Category $cat, WikiPage $page
	) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		# Is anyone watching the category?
		if (
			self::getWatchedItemStore()
			->countWatchers( $cat->getTitle() ) > 0
		) {
			# Send them a notification!
			$user = User::newFromId( $page->getUser() );

			EchoEvent::create( [
				'type' => 'categorywatch-add',
				'title' => $cat->getTitle(),
				'agent' => $user,
				'extra' => [
					'pageid' => $page->getId(),
					'revid' => $page->getRevision()->getId(),
				],
			] );
		}
	}

	/**
	 * Hook for page being taken out of a category.
	 *
	 * @param Category $cat that page is being removed from
	 * @param WikiPage $page that is being removed
	 * @param int $id that this happened in. (not given pre 1.27ish)
	 */
	public static function onCategoryAfterPageRemoved(
		Category $cat, WikiPage $page, $id = 0
	) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		# Is anyone watching the category?
		if (
			self::getWatchedItemStore()
			->countWatchers( $cat->getTitle() ) > 0
		) {
			# Send them a notification!
			$user = User::newFromId( $page->getUser() );

			$revId = null;
			$rev = $page->getRevision();
			if ( $rev ) {
				$revId = $rev->getId();
			}
			EchoEvent::create( [
				'type' => 'categorywatch-remove',
				'title' => $cat->getTitle(),
				'agent' => $user,
				'extra' => [
					'pageid' => $page->getId(),
					'revid' => $revId
				],
			] );
		}
	}

	/**
	 * Find the watchers for a title
	 *
	 * @param Title $target to check
	 *
	 * @return array
	 */
	protected static function getWatchers( Title $target ) {
		$dbr = wfGetDB( DB_SLAVE );
		$return = $dbr->selectFieldValues(
			'watchlist',
			'wl_user',
			[
				'wl_namespace' => $target->getNamespace(),
				'wl_title' => $target->getDBkey(),
			],
			__METHOD__
		);

		return array_map( function ( $id ) {
			return User::newFromID( $id );
		}, $return );
	}

	/**
	 * Get users that should be notified for this event.
	 *
	 * @param EchoEvent $event to be looked at
	 * @return array
	 */
	public static function userLocater( EchoEvent $event ) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		return self::getWatchers( $event->getTitle() );
	}

	/**
	 * Filter out the person performing the action
	 *
	 * @param EchoEvent $event to be looked at
	 * @return array
	 */
	public static function userFilter( EchoEvent $event ) {
		wfDebugLog( 'CategoryWatch', __METHOD__ );
		return [ $event->getAgent() ];
	}
}
