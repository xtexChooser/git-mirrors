<?php
class LoginNotifyPresentationModel extends EchoEventPresentationModel {

	/**
	 * Show a lock icon, for account security.
	 *
	 * @return String Name of icon
	 */
	public function getIconType() {
		return 'LoginNotify-lock';
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
	 * (For grep) This uses i18n messages:
	 *	notification-header-login-fail-known
	 *	notification-header-login-fail-new
	 *	notification-header-login-success
	 *
	 * @return Message
	 */
	public function getHeaderMessage() {
		return parent::getHeaderMessage()->numParams(
			$this->event->getExtraParam( 'count', 0 )
		);
	}

	/**
	 * @todo FIXME Unclear if this is a good idea
	 */
	public function getSecondaryLinks() {
		return array( $this->getAgentLink() );
	}
}
