<?php
class LoginNotifyPresentationModel extends EchoEventPresentationModel {

	/**
	 * Show an user avatar.
	 *
	 * @return String Name of icon
	 */
	public function getIconType() {
		return 'LoginNotify-user-avatar';
	}

	/**
	 * Nothing really to link to
	 *
	 * @return boolean false to disable link
	 */
	public function getPrimaryLink() {
		return false;
	}

	/**
	 * Include the number of attempts in the message
	 *
	 * @return Message
	 */
	public function getHeaderMessage() {
		// Check if we got a bundled notification with a 'count' param
		// 'count' param is set when we have a failed login attempt
		$msg = '';
		switch ( $this->event->getType() ) {
			case 'login-fail-known':
				if ( $this->event->getExtraParam( 'count', 0 ) > 0 ) {
					$msg = $this->msg( 'notification-known-header-login-fail' );
					$msg->params( $this->event->getExtraParam( 'count', 0 ) );
					return $msg;
				} else {
					$msg = $this->msg( 'notification-header-login-success' );
					$msg->params( $this->event->getExtraParam( 'count', 0 ) );
					return $msg;
				}
				break;
			case 'login-fail-new':
				if ( $this->isBundled() && ( $this->event->getExtraParam( 'count', 0 ) > 0 ) ) {
					$msg = $this->msg( 'notification-new-bundled-header-login-fail' );
					$msg->params( $this->event->getExtraParam( 'count', 0 ) );
					return $msg;
				} elseif ( $this->event->getExtraParam( 'count', 0 ) > 0 ) {
					$msg = $this->msg( 'notification-new-unbundled-header-login-fail' );
					return $msg;
				} else {
					$msg = $this->msg( 'notification-header-login-success' );
					return $msg;
				}
				break;
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
