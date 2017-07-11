<?php

namespace LoginNotify;

use Exception;
use Job;
use Title;
use User;

/**
 * Class DeferredChecksJob
 * @package LoginNotify
 */
class DeferredChecksJob extends Job {
	const TYPE_LOGIN_FAILED = 'failed';
	const TYPE_LOGIN_SUCCESS = 'success';

	/**
	 * @param Title $title
	 * @param array|bool $params
	 */
	public function __construct( Title $title, $params = false ) {
		parent::__construct( 'LoginNotifyChecks', $title, $params );
	}

	/**
	 * Run the job
	 * @return bool Success
	 */
	public function run() {
		$checkType = $this->params['checkType'];
		$userId = $this->params['userId'];
		$user = User::newFromId( $userId );
		if ( !$user ) {
			throw new Exception( "Can't find user for user id=" . print_r( $userId, true ) );
		}
		if ( !isset( $this->params['subnet'] ) || !is_string( $this->params['subnet'] ) ) {
			throw new Exception( __CLASS__
				. " expected to receive a string parameter 'subnet', got "
				. print_r( $this->params['subnet'], true )
			);
		}
		$subnet = $this->params['resultSoFar'];
		if ( !isset( $this->params['resultSoFar'] ) || !is_string( $this->params['resultSoFar'] ) ) {
			throw new Exception( __CLASS__
				. " expected to receive a string parameter 'resultSoFar', got "
				. print_r( $this->params['resultSoFar'], true )
			);
		}
		$resultSoFar = $this->params['resultSoFar'];

		$loginNotify = new LoginNotify();

		switch ( $checkType ) {
			case self::TYPE_LOGIN_FAILED:
				$loginNotify->recordFailureDeferred( $user, $subnet, $resultSoFar );
				break;
			case self::TYPE_LOGIN_SUCCESS:
				$loginNotify->sendSuccessNoticeDeferred( $user, $subnet, $resultSoFar );
				break;
			default:
				throw new Exception( 'Unknown check type ' . print_r( $checkType, true ) );
		}

		return true;
	}
}
