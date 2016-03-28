<?php
class LoginNotifyFormatter extends EchoBasicFormatter {

	/**
	 * Add the number of attempts as a param to the email.
	 *
	 * @param $event EchoEvent
	 * @param $param
	 * @param $message Message
	 * @param $user User
	 */
	protected function processParam( $event, $param, $message, $user ) {
		if ( $param === 'count' ) {
			$message->params( $event->getExtraParam( 'count' ) );
		} else {
			parent::processParam( $event, $param, $message, $user );
		}
	}
}
