<?php
/**
 * Body of LoginNotify extension
 *
 * @file
 * @ingroup Extensions
 */

use MediaWiki\Auth\AuthenticationResponse;

class LoginNotifyHooks {

	const OPTIONS_FAKE_TRUTH = 2;
	const OPTIONS_FAKE_FALSE = 'fake-false';

	/**
	 * Add LoginNotify events to Echo
	 *
	 * @param $notifications array of Echo notifications
	 * @param $notificationCategories array of Echo notification categories
	 * @param $icons array of icon details
	 * @return bool
	 */
	public static function onBeforeCreateEchoEvent(
		&$notifications,
		&$notificationCategories,
		&$icons
	) {
		global $wgLoginNotifyEnableOnSuccess;

		$icons['LoginNotify-lock'] = [
			'path' => 'LoginNotify/Lock.svg'
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
			'email-subject-message' => 'notification-loginnotify-login-fail-email-subject',
			// FIXME Should count be a parameter
			'email-subject-params' => [ 'agent', 'count' ],
			'email-body-batch-params' => [ 'agent', 'count' ],
			// FIXME is it ok not to set batch email messages, since
			// we have immediate flag?
			'icon' => 'LoginNotify-lock',
			'immediate' => true,
			'formatter-class' => 'LoginNotifyFormatter',
		];
		$notifications['login-fail-new'] = [
			'email-body-batch-message' => 'notification-loginnotify-login-fail-new-emailbatch'
		] + $loginBase;
		$notifications['login-fail-known'] = [
			'email-body-batch-message' => 'notification-loginnotify-login-fail-known-emailbatch'
		] + $loginBase;
		if ( $wgLoginNotifyEnableOnSuccess ) {
			$notificationCategories['login-success'] = [
				'priority' => 7,
				'tooltip' => 'echo-pref-tooltip-login-success',
			];
			$notifications['login-success'] = [
				'category' => 'login-success',
				'email-subject-message' => 'notification-loginnotify-login-success-email-subject',
				'email-body-batch-message' => 'notification-loginnotify-login-success-emailbatch',
				'email-body-batch-params' => [ 'agent' ],
				// FIXME title-message. What is its purpose??
			] + $loginBase;
		}

		return true;
	}

	/**
	 * Old hook for pre 1.27 or wikis with auth manager disabled.
	 *
	 * @todo Doesn't catcha captcha or throttle failures
	 * @param $user User User in question
	 * @param $pass String password
	 * @param $retval int LoginForm constant (e.g. LoginForm::SUCCESS)
	 * @return bool Standard hook return
	 */
	public static function onLoginAuthenticateAudit( User $user, $pass, $retval ) {
		if ( !class_exists( 'EchoEvent' ) ) {
			throw new FatalError( "LoginNotify extension requires the Echo extension to be installed" );
		}
		if ( $retval === LoginForm::WRONG_PASS ) {
			self::doFailedLogin( $user );
		} elseif ( $retval === LoginForm::SUCCESS ) {
			self::doSuccessfulLogin( $user );
		}
	}

	/**
	 * Hook for login auditing post 1.27
	 *
	 * @param $ret AuthenticationResponse Is login succesful?
	 * @param $user User|null User object on successful auth
	 * @param $username String Username for failed attempts.
	 */
	public static function onAuthManagerLoginAuthenticateAudit(
		AuthenticationResponse $ret, $user, $username
	) {
		if ( !class_exists( 'EchoEvent' ) ) {
			throw new FatalError( "LoginNotify extension requires the Echo extension to be installed" );
		}
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

	public static function doSuccessfulLogin( User $user ) {
		$loginNotify = new LoginNotify();
		$loginNotify->clearCounters( $user );
		$loginNotify->sendSuccessNotice( $user );
		$loginNotify->setCurrentAddressAsKnown( $user );
	}

	public static function doFailedLogin( User $user ) {
		$loginNotify = new LoginNotify();
		$loginNotify->recordFailure( $user );
	}

	/**
	 * Set a cookie saying this is a known computer when creating an account.
	 *
	 * @todo This still sets cookies if user creates account well logged in as someone else.
	 * @param User $user
	 * @param boolean $byMail Account created by email
	 */
	public static function onAddNewAccount( User $user, $byMail ) {
		if ( !$byMail ) {
			$loginNotify = new LoginNotify();
			$loginNotify->setCurrentAddressAsKnown( $user );
		}
	}

	/**
	 * Hook for loading options.
	 *
	 * This is a bit hacky. Used to be able to set a different
	 * default for admins then other users
	 *
	 * @param $user User
	 * @param &$options array
	 * @return bool
	 */
	public static function onUserLoadOptions( User $user, array &$options ) {
		global $wgLoginNotifyEnableForPriv;
		if ( !is_array( $wgLoginNotifyEnableForPriv ) ) {
			return true;
		}

		if ( !self::isUserOptionOverriden( $user, $wgLoginNotifyEnableForPriv ) ) {
			return true;
		}

		$defaultOpts = User::getDefaultOptions();
		$optionsToCheck = self::getOverridenOptions();

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
	 * default for admins then other users. Since admins are higher value
	 * targets, it may make sense to have notices enabled by default for
	 * them, but disabled for normal users.
	 *
	 * @todo This is a bit icky. Need to decide if we really want to do this.
	 * @todo If someone explicitly enables, gets admin rights, gets de-admined,
	 *   this will then disable the preference, which is definitely non-ideal.
	 * @param $user User
	 * @param &$options array
	 * @return bool
	 */
	public function onUserSaveOptions( User $user, array &$options ) {
		global $wgLoginNotifyEnableForPriv;
		$optionsToCheck = self::getOverridenOptions();
		$defaultOpts = User::getDefaultOptions();
		if ( !self::isUserOptionOverriden( $user, $wgLoginNotifyEnableForPriv ) ) {
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
	private static function getOverridenOptions() {
		// For login-success, it makes most sense to email
		// people about it, but auto-subscribing people to email
		// is a bit icky as nobody likes to be spammed.
		return [
			'echo-subscriptions-web-login-fail',
			'echo-subscriptions-web-login-success'
		];
	}

	private static function isUserOptionOverriden( User $user ) {
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
