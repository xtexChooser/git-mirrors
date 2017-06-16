<?php

namespace LoginNotify;

use EchoEventPresentationModel;
use Message;
use SpecialPage;

class PresentationModel extends EchoEventPresentationModel {

	/**
	 * Show an user avatar.
	 *
	 * @return string Name of icon
	 */
	public function getIconType() {
		return 'LoginNotify-user-avatar';
	}

	/**
	 * Link to help page on mediawiki
	 *
	 * @return array URL to link to
	 */
	public function getPrimaryLink() {
		return [
			'url' =>  'https://mediawiki.org/wiki/Help:Login_notifications',
			'label' => $this->msg( 'loginnotify-primary-link' )->text()
		];
	}

	/**
	 * Define the email subject string
	 *
	 * @return string Message string for email subject
	 */
	public function getSubjectMessage() {
		switch ( $this->event->getType() ) {
			case 'login-fail-known':
			case 'login-fail-new':
				$msg = $this->msg( 'notification-loginnotify-login-fail-email-subject' );
				$msg->params( $this->getUser()->getName() );
				$msg->params( $this->event->getExtraParam( 'count', 0 ) );
				break;
			default:
				$msg = $this->msg( 'notification-loginnotify-login-success-email-subject' );
				$msg->params( $this->getUser()->getName() );
				break;
		}
		return $msg;
	}

	/**
	 * Include the number of attempts in the message if needed
	 *
	 * @return Message
	 */
	public function getHeaderMessage() {
		switch ( $this->event->getType() ) {
			case 'login-fail-known':
				$msg = $this->msg( 'notification-known-header-login-fail' );
				$msg->params( $this->event->getExtraParam( 'count', 0 ) );
				break;
			case 'login-fail-new':
				if ( $this->isBundled() ) {
					$msg = $this->msg( 'notification-new-bundled-header-login-fail' );
					$msg->params( $this->event->getExtraParam( 'count', 0 ) );
				} else {
					$msg = $this->msg( 'notification-new-unbundled-header-login-fail' );
					$msg->params( $this->event->getExtraParam( 'count', 0 ) );
				}
				break;
			default:
				$msg = $this->msg( 'notification-header-login-success' );
		}
		return $msg;
	}

	/**
	 * Get links to be used in the notification
	 *
	 * @return array Link to Special:ChangePassword
	 */
	public function getSecondaryLinks() {
		$changePasswordLink = [
			'url' => SpecialPage::getTitleFor( 'ChangePassword' )->getFullURL(),
			'label' => $this->msg( 'changepassword' )->text(),
			'description' => '',
			'icon' => 'lock',
			'prioritized' => true,
		];

		return [ $changePasswordLink ];
	}
}
