<?php
/**
 * Body of LoginNotify extension
 *
 * @file
 * @ingroup Extensions
 */

namespace LoginNotify;

use EchoAttributeManager;
use EchoEvent;
use LoginForm;
use MediaWiki\Auth\AuthenticationResponse;
use User;

class Hooks {

	const OPTIONS_FAKE_TRUTH = 2;
	const OPTIONS_FAKE_FALSE = 'fake-false';

	/**
	 * Add LoginNotify events to Echo
	 *
	 * @param string[] &$notifications Array of Echo notifications
	 * @param string[] &$notificationCategories Array of Echo notification categories
	 * @param string[] &$icons Array of icon details
	 * @return bool
	 */
	public static function onBeforeCreateEchoEvent(
		array &$notifications,
		array &$notificationCategories,
		array &$icons
	) {
		global $wgLoginNotifyEnableOnSuccess;

		$icons['LoginNotify-user-avatar'] = [
			'path' => 'LoginNotify/UserAvatar.svg'
		];

		$notificationCategories['login-fail'] = [
			'priority' => 7,
			'tooltip' => 'echo-pref-tooltip-login-fail',
		];

		$loginBase = [
			EchoAttributeManager::ATTR_LOCATORS => [
				'EchoUserLocator::locateEventAgent'
			],
			'category' => 'login-fail',
			'group' => 'negative',
			'presentation-model' => 'LoginNotifyPresentationModel',
			// fixme, what does this actually do?
			'title-message' => 'loginnotify-login-fail',
			'title-params' => [],
			// FIXME Should count be a parameter
			'email-subject-params' => [ 'agent', 'count' ],
			'email-body-batch-params' => [ 'agent', 'count' ],
			// FIXME is it ok not to set batch email messages, since
			// we have immediate flag?
			'icon' => 'LoginNotify-user-avatar',
			'immediate' => true,
		];
		$notifications['login-fail-new'] = [
			'bundle' => [
				'web' => true,
				'expandable' => false
			]
		] + $loginBase;
		$notifications['login-fail-known'] = [
			'bundle' => [
				'web' => true,
				'expandable' => false
			]
		] + $loginBase;
		if ( $wgLoginNotifyEnableOnSuccess ) {
			$notificationCategories['login-success'] = [
				'priority' => 7,
				'tooltip' => 'echo-pref-tooltip-login-success',
			];
			$notifications['login-success'] = [
				'category' => 'login-success',
				// FIXME title-message. What is its purpose??
			] + $loginBase;
		}

		return true;
	}

	/**
	 * @param EchoEvent $event
	 * @param string $bundleString
	 * @return bool
	 */
	public static function onEchoGetBundleRules( EchoEvent $event, &$bundleString ) {
		switch ( $event->getType() ) {
			case 'login-fail-new':
				$bundleString = 'login-fail';
				break;
		}
		return true;
	}

	/**
	 * Old hook for pre 1.27 or wikis with auth manager disabled.
	 *
	 * @todo Doesn't catch CAPTCHA or throttle failures
	 *
	 * @param User $user User in question.
	 * @param string $pass The password (parameter not used).
	 * @param integer $retval A LoginForm constant (e.g. LoginForm::SUCCESS).
	 */
	public static function onLoginAuthenticateAudit( User $user, $pass, $retval ) {
		if ( $retval === LoginForm::WRONG_PASS ) {
			self::doFailedLogin( $user );
		} elseif ( $retval === LoginForm::SUCCESS ) {
			self::doSuccessfulLogin( $user );
		}
	}

	/**
	 * Hook for login auditing post 1.27
	 *
	 * @param AuthenticationResponse $ret Is login successful?
	 * @param User|null $user User object on successful auth
	 * @param string $username Username for failed attempts.
	 */
	public static function onAuthManagerLoginAuthenticateAudit(
		AuthenticationResponse $ret, $user, $username
	) {
		if ( $user ) {
			$userObj = $user;
		} else {
			$userObj = User::newFromName( $username, 'usable' );
		}
		if ( !$userObj ) {
			return;
		}

		if ( $ret->status === AuthenticationResponse::PASS ) {
			self::doSuccessfulLogin( $userObj );
		} elseif ( $ret->status === AuthenticationResponse::FAIL ) {
			self::doFailedLogin( $userObj );
		}
		// Other statuses include Abstain, Redirect, or UI. We ignore such
		// statuses.
	}

	/**
	 * Handle a successful login (clear the attempt counter, send a notice, and record the
	 * current IP address as known).
	 *
	 * @param User $user The user who logged in.
	 */
	public static function doSuccessfulLogin( User $user ) {
		$loginNotify = new LoginNotify();
		$loginNotify->clearCounters( $user );
		$loginNotify->sendSuccessNotice( $user );
		$loginNotify->setCurrentAddressAsKnown( $user );
	}

	/**
	 * Handle a failed login (record the failure).
	 *
	 * @param User $user The user that failed to log in.
	 */
	public static function doFailedLogin( User $user ) {
		$loginNotify = new LoginNotify();
		$loginNotify->recordFailure( $user );
	}

	/**
	 * Deprecated since v1.27
	 *
	 * Set a cookie saying this is a known computer when creating an account.
	 *
	 * @todo This still sets cookies if user creates an account while logged in as someone else.
	 * @param User $user The user that has been created.
	 * @param boolean $byMail Account created by email
	 */
	public static function onAddNewAccount( User $user, $byMail ) {
		if ( !$byMail ) {
			$loginNotify = new LoginNotify();
			$loginNotify->setCurrentAddressAsKnown( $user );
		}
	}

	/**
	 * Hook for new account creation since v1.27
	 *
	 * Called immediately after a local user has been created and saved to the database
	 *
	 * @todo This still sets cookies if user creates account well logged in as someone else.
	 * @param User $user User created
	 * @param boolean $autocreated Whether this was an auto-created account
	 */
	public static function onLocalUserCreated( $user, $autocreated ) {
		if ( !$autocreated ) {
			$loginNotify = new LoginNotify();
			$loginNotify->setCurrentAddressAsKnown( $user );
		}
	}

	/**
	 * Hook for loading options.
	 *
	 * This is a bit hacky. Used to be able to set a different
	 * default for admins than other users
	 *
	 * @param User $user The user in question.
	 * @param mixed[] &$options The options.
	 * @return bool
	 */
	public static function onUserLoadOptions( User $user, array &$options ) {
		global $wgLoginNotifyEnableForPriv;
		if ( !is_array( $wgLoginNotifyEnableForPriv ) ) {
			return true;
		}

		if ( !self::isUserOptionOverridden( $user ) ) {
			return true;
		}

		$defaultOpts = User::getDefaultOptions();
		$optionsToCheck = self::getOverriddenOptions();

		foreach ( $optionsToCheck as $opt ) {
			if ( $options[$opt] === self::OPTIONS_FAKE_FALSE ) {
				$options[$opt] = '0';
			}
			if ( $defaultOpts[$opt] !== false ) {
				continue;
			}
			if ( $options[$opt] === false ) {
				$options[$opt] = self::OPTIONS_FAKE_TRUTH;
			}
		}
		return true;
	}

	/**
	 * Hook for saving options.
	 *
	 * This is a bit hacky. Used to be able to set a different
	 * default for admins than other users. Since admins are higher value
	 * targets, it may make sense to have notices enabled by default for
	 * them, but disabled for normal users.
	 *
	 * @todo This is a bit icky. Need to decide if we really want to do this.
	 * @todo If someone explicitly enables, gets admin rights, gets de-admined,
	 *   this will then disable the preference, which is definitely non-ideal.
	 * @param User $user The user that is being saved.
	 * @param mixed[] &$options The options.
	 * @return bool
	 */
	public static function onUserSaveOptions( User $user, array &$options ) {
		$optionsToCheck = self::getOverriddenOptions();
		$defaultOpts = User::getDefaultOptions();
		if ( !self::isUserOptionOverridden( $user ) ) {
			return true;
		}
		foreach ( $optionsToCheck as $opt ) {
			if ( $defaultOpts[$opt] !== false ) {
				continue;
			}

			if ( $options[$opt] === self::OPTIONS_FAKE_TRUTH ) {
				$options[$opt] = false;
			}
			if ( $options[$opt] !== self::OPTIONS_FAKE_TRUTH
				&& $options[$opt]
			) {
				// Its checked on the form. Keep at default
			}

			if ( !$options[$opt] ) {
				// Somehow this means it got unchecked on form
				$options[$opt] = self::OPTIONS_FAKE_FALSE;
			}
		}
		return true;
	}

	/**
	 * Helper for onUser(Load|Save)Options
	 *
	 * @return array Which option keys to check
	 */
	private static function getOverriddenOptions() {
		// For login-success, it makes most sense to email
		// people about it, but auto-subscribing people to email
		// is a bit icky as nobody likes to be spammed.
		return [
			'echo-subscriptions-web-login-fail',
			'echo-subscriptions-web-login-success'
		];
	}

	private static function isUserOptionOverridden( User $user ) {
		global $wgLoginNotifyEnableForPriv;
		// Note: isAllowedAny calls into session for per-session restrictions,
		// which we do not want to take into account, and more importantly
		// causes an infinite loop.
		$rights = User::getGroupPermissions( $user->getEffectiveGroups() );
		if ( !array_intersect( $rights, $wgLoginNotifyEnableForPriv ) ) {
			// Not a user we care about.
			return false;
		}
		return true;
	}
}
