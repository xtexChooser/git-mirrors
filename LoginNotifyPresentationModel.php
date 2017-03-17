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
