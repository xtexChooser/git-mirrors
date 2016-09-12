<?php
/**
 * Body of LoginNotify extension
 *
 * @file
 * @ingroup Extensions
 */

use MediaWiki\Logger\LoggerFactory;
use Psr\Log\LoggerInterface;
use Psr\Log\LoggerAwareInterface;

/**
 * Handle sending notifications on login from unknown source.
 *
 * @author Brian Wolff
 */
class LoginNotify implements LoggerAwareInterface {

	const COOKIE_NAME = 'mw_prevLogin';
	const NO_INFO_AVAILABLE = 2;

	/** @var BagOStuff */
	private $cache;
	/** @var Config */
	private $config;
	/** @var LoggerInterface Usually instance of LoginNotify log */
	private $log;

	/**
	 * Constructor
	 *
	 * @param $cfg Config Optional. Set if you have handy.
	 * @param $cache BagOStuff Optional. Only set if you want to override default
	 *   caching behaviour.
	 */
	public function __construct( Config $cfg = null, BagOStuff $cache = null ) {
		if ( !$cache ) {
			$cache = ObjectCache::getLocalClusterInstance();
		}
		if ( !$cfg ) {
			$cfg = RequestContext::getMain()->getConfig();
		}
		$this->cache = $cache;
		$this->config = $cfg;

		if ( $this->config->get( 'LoginNotifySecretKey' ) !== null ) {
			$this->secret = $this->config->get( 'LoginNotifySecretKey' );
		} else {
			$globalSecret = $this->config->get( 'SecretKey' );
			$this->secret = hash( 'sha256', $globalSecret + 'LoginNotify' );
		}

		$log = LoggerFactory::getInstance( 'LoginNotify' );
		$this->log = $log;
	}

	public function setLogger( LoggerInterface $logger ) {
		$this->log = $logger;
	}

	/**
	 * Get just network part of an IP (assuming /24 or /64)
	 *
	 * @param String $ip Either IPv4 or IPv6 address
	 * @return string Just the network part (e.g. 127.0.0.)
	 * @throws UnexpectedValueException If given something not an IP
	 * @throws Exception If regex totally fails (Should never happen)
	 */
	private function getIPNetwork( $ip ) {
		$ip = IP::sanitizeIP( $ip );
		if ( IP::isIPv6( $ip ) ) {
			// Match against the /64
			$subnetRegex = '/[0-9A-F]+:[0-9A-F]+:[0-9A-F]+:[0-9A-F]+$/i';
		} elseif ( IP::isIPv4( $ip ) ) {
			// match against the /24
			$subnetRegex = '/\d+$/';
		} else {
			throw new UnexpectedValueException( "Unrecognized IP address: $ip" );
		}
		$prefix = preg_replace( $subnetRegex, '', $ip );
		if ( !is_string( $prefix ) ) {
			throw new Exception( __METHOD__ . " Regex failed!?" );
		}
		return $prefix;
	}

	/**
	 * Is the current computer known to be used by the given user
	 *
	 * @param $user User User in question
	 * @return boolean true if the user has used this computer before
	 */
	private function isFromKnownIP( User $user ) {
		$cookieResult = $this->checkUserInCookie( $user );
		if ( $cookieResult === true ) {
			// User has cookie
			return true;
		}

		$cacheResult = $this->checkUserInCache( $user );
		if ( $cacheResult === true ) {
			return true;
		}

		$cuResult = $this->checkUserInCheckUser( $user );
		if ( $cuResult === true ) {
			return true;
		}

		// If we have no check user data for the user, and there was
		// no cookie supplied, just pass the user in, since we don't have
		// enough info to determine if from known ip.
		// FIXME: Does this make sense
		if (
			$cuResult === self::NO_INFO_AVAILABLE &&
			$cookieResult === self::NO_INFO_AVAILABLE &&
			$cacheResult === self::NO_INFO_AVAILABLE
		) {
			// We have to be careful here. Whether $cookieResult is
			// self::NO_INFO_AVAILABLE, is under control of the attacker.
			// If checking CheckUser is disabled, then we should not
			// hit this branch.

			$this->log->info( "Assuming {user} is from known IP since no info available", [
				'method' => __METHOD__,
				'user' => $user->getName()
			] );
			return true;
		}

		return false;
	}

	/**
	 * Check if we cached this user's ip address from last login.
	 *
	 * @param $user User User in question.
	 * @return Mixed true, false or self::NO_INFO_AVAILABLE.
	 */
	private function checkUserInCache( User $user ) {
		$ipPrefix = $this->getIPNetwork( $user->getRequest()->getIP() );
		$key = $this->getKey( $user, 'prevSubnet' );
		$res = $this->cache->get( $key );
		if ( $res !== false ) {
			return $res === $ipPrefix;
		}
		return self::NO_INFO_AVAILABLE;
	}

	/**
	 * Is the subnet of the current IP in the check user data for the user.
	 *
	 * If CentralAuth is installed, this will check not only the current wiki,
	 * but also the ten wikis where user has most edits on.
	 *
	 * @param $user User User in question.
	 * @return Mixed true, false or self::NO_INFO_AVAILABLE.
	 */
	private function checkUserInCheckUser( User $user ) {
		if ( !$this->config->get( 'LoginNotifyCheckKnownIPs' )
			|| !class_exists( 'CheckUser' )
		) {
			// Check user checks disabled.
			// Note: Its important this be false and not self::NO_INFO_AVAILABLE.
			return false;
		}

		$haveAnyInfo = false;
		$prefix = $this->getIPNetwork( $user->getRequest()->getIP() );

		$dbr = wfGetDB( DB_SLAVE );
		$localResult = $this->checkUserInCheckUserQuery( $user->getId(), $prefix, $dbr );
		if ( $localResult ) {
			return true;
		}

		if ( !$haveAnyInfo ) {
			$haveAnyInfo = $this->checkUserInCheckUserAnyInfo( $user->getId(), $dbr );
		}

		// Also check checkuser table on the top ten wikis where this user has
		// edited the most. We only do top ten, to limit the worst-case where the
		// user has accounts on 800 wikis.
		if ( class_exists( 'CentralAuthUser' ) ) {
			$wikisByEditCounts = [];
			$globalUser = CentralAuthUser::getInstance( $user );
			if ( $globalUser->exists() ) {
				// This is expensive. However, On WMF wikis, probably
				// already done as part of password complexity check, and
				// will be cached.
				$info = $globalUser->queryAttached();
				// already checked the local wiki.
				unset( $info[wfWikiId()] );
				usort( $info,
					function( $a, $b ) {
						// descending order
						return $b['editCount'] - $a['editCount'];
					}
				);
				$count = 0;
				foreach ( $info as $wiki => $localInfo ) {
					if ( $count > 10 || $localInfo['editCount'] < 1 ) {
						break;
					}
					$lb = wfGetLB( $wiki );
					$dbrLocal = $lb->getConnection( DB_SLAVE, [], $wiki );

					if ( !$this->hasCheckUserTables( $dbrLocal ) ) {
						// Skip this wiki, no checkuser table.
						$lb->reuseConnection( $dbrLocal );
						continue;
					}
					// FIXME The case where there are no CU entries for
					// this user.
					$res = $this->checkUserInCheckUserQuery(
						$localInfo['id'],
						$prefix,
						$dbrLocal
					);

					if ( $res ) {
						$lb->reuseConnection( $dbrLocal );
						return true;
					}
					if ( !$haveAnyInfo ) {
						$haveAnyInfo = $this->checkUserInCheckUserAnyInfo( $user->getId(), $dbr );
					}
					$lb->reuseConnection( $dbrLocal );
					$count++;
				}
			}
		}
		if ( !$haveAnyInfo ) {
			return self::NO_INFO_AVAILABLE;
		}
		return false;
	}

	/**
	 * Actually do the query of the check user table.
	 *
	 * @note This catches and ignores database errors.
	 * @param $userId int User id number (Not neccesarily for the local wiki)
	 * @param $ipFragment string Prefix to match against cuc_ip (from $this->getIPNetwork())
	 * @param $dbr DatabaseBase A database connection (possibly foreign)
	 * @return boolean If $ipFragment is in check user db
	 */
	private function checkUserInCheckUserQuery( $userId, $ipFragment, DatabaseBase $dbr ) {
		// For some unknown reason, the index is on
		// (cuc_user, cuc_ip, cuc_timestamp), instead of
		// cuc_ip_hex which would be ideal.
		// user-agent might also be good to look at,
		// but no index on that.
		$IPHasBeenUsedBefore = $dbr->selectField(
			'cu_changes',
			'1',
			[
				'cuc_user' => $userId,
				'cuc_ip ' . $dbr->buildLike(
					$ipFragment,
					$dbr->anyString()
				)
			],
			__METHOD__
		);
		return $IPHasBeenUsedBefore;
	}

	/**
	 * Check if we have any check user info for this user
	 *
	 * If we have no info for user, we maybe don't treat it as
	 * an unknown IP, since user has no known IPs.
	 *
	 * @todo FIXME Does this behaviour make sense, esp. with cookie check?
	 * @param $userId int User id number (possibly on foreign wiki)
	 * @param $dbr DatabaseBase DB connection (possibly to foreign wiki)
	 */
	private function checkUserInCheckUserAnyInfo( $userId, DatabaseBase $dbr ) {
		// Verify that we actually have IP info for
		// this user.
		// @todo: Should this instead be if we have a
		// a certain number of checkuser entries for this
		// user. Or maybe it should be if we have at least
		// 2 different IPs for this user. Or something else.
		$haveIPInfo = $dbr->selectField(
			'cu_changes',
			'1',
			[
				'cuc_user' => $userId
			],
			__METHOD__
		);

		return $haveIPInfo;
	}

	/**
	 * Does this wiki have a checkuser table?
	 *
	 * @param DatabaseBase $dbr Database to check
	 * @return boolean
	 */
	private function hasCheckUserTables( DatabaseBase $dbr ) {
		if ( !$dbr->tableExists( 'cu_changes' ) ) {
			$this->log->warning( "LoginNotify: No checkuser table on {wikiId}", [
				'method' => __METHOD__,
				'wikiId' => $dbr->getWikiID()
			] );
			return false;
		}
		return true;
	}

	/**
	 * Give the user a cookie saying that they've previously logged in from this computer.
	 *
	 * @note If user already has a cookie, this will refresh it.
	 * @param $user User User in question who just logged in.
	 */
	private function setLoginCookie( User $user ) {
		$cookie = $this->getPrevLoginCookie( $user->getRequest() );
		list( , $newCookie ) = $this->checkAndGenerateCookie( $user, $cookie );
		$expire = time() + $this->config->get( 'LoginNotifyCookieExpire' );
		$resp = $user->getRequest()->response();
		$resp->setCookie(
			self::COOKIE_NAME,
			$newCookie,
			$expire,
			[
				'domain' => $this->config->get( 'LoginNotifyCookieDomain' ),
				// Allow sharing this cookie between wikis
				'prefix' => ''
			]
		);
	}

	/**
	 * Give the user a cookie and cache address in memcache
	 *
	 * It is expected this be called upon successful log in.
	 *
	 * @param $user User The user in question.
	 */
	public function setCurrentAddressAsKnown( User $user ) {
		$this->cacheLoginIP( $user );
		$this->setLoginCookie( $user );
	}

	/**
	 * Cache the current IP subnet as being known location for user
	 *
	 * @param $user User
	 */
	private function cacheLoginIP( User $user ) {
		// For simplicity, this only stores the last IP subnet used.
		// Its assumed that most of the time, we'll be able to rely on
		// the cookie or checkuser data.
		$expiry = $this->config->get( 'LoginNotifyCacheLoginIPExpiry' );
		if ( $expiry !== false ) {
			$ipPrefix = $this->getIPNetwork( $user->getRequest()->getIP() );
			$key = $this->getKey( $user, 'prevSubnet' );
			$res = $this->cache->set( $key, $ipPrefix, $expiry );
		}
	}

	/**
	 * Check if a certain user is in the cookie.
	 *
	 * @param $user User User in question
	 * @return Mixed true, false or self::NO_INFO_AVAILABLE.
	 */
	private function checkUserInCookie( User $user ) {
		$cookie = $this->getPrevLoginCookie( $user->getRequest() );
		if ( $cookie === '' ) {
			// FIXME, does this really make sense?
			return self::NO_INFO_AVAILABLE;
		}
		list( $userKnown, ) = $this->checkAndGenerateCookie( $user, $cookie );
		return $userKnown;
	}

	/**
	 * Get the cookie with previous login names in it
	 *
	 * @param WebRequest
	 * @return String The cookie. Empty string if no cookie.
	 */
	private function getPrevLoginCookie( WebRequest $req ) {
		return $req->getCookie( self::COOKIE_NAME, '', '' );
	}

	/**
	 * Check if user is in cookie, and generate a new cookie with user record
	 *
	 * When generating a new cookie, it will add the current user to the top,
	 * remove any previous instances of the current user, and remove older user
	 * references, if there is too many records.
	 *
	 * @param $user User User that person is attempting to log in as
	 * @param $cookie String A cookie, which has records separated by '!'
	 */
	private function checkAndGenerateCookie( User $user, $cookie ) {
		$userSeenBefore = false;
		if ( $cookie === '' ) {
			$cookieRecords = [];
		} else {
			$cookieRecords = explode( '.', $cookie );
		}
		$newCookie = $this->generateUserCookieRecord( $user->getName() );
		$maxCookieRecords = $this->config->get( 'LoginNotifyMaxCookieRecords' );

		$totalCookieRecord = count( $cookieRecords );
		for ( $i = 0; $i < $totalCookieRecord; $i++ ) {
			if ( !$this->validateCookieRecord( $cookieRecords[$i] ) ) {
				// Skip invalid or old cookie records.
				continue;
			}
			$curUser = $this->checkUserRecordGivenCookie( $user, $cookieRecords[$i] );
			$userSeenBefore = $userSeenBefore || $curUser;
			if ( $i < $maxCookieRecords && !$curUser ) {
				$newCookie .= '.' . $cookieRecords[$i];
			}
		}
		return [ $userSeenBefore, $newCookie ];
	}

	/**
	 * See if a specific cookie record is for a specific user.
	 *
	 * Cookie record format is: Year - 32-bit salt - hash
	 * where hash is sha1-HMAC of username + | + year + salt
	 * Salt and hash is base 36 encoded.
	 *
	 * The point of the salt is to ensure that a given user creates
	 * different cookies on different machines, so that nobody
	 * can after the fact figure out a single user has used both
	 * machines.
	 */
	private function checkUserRecordGivenCookie( User $user, $cookieRecord ) {
		if ( !$this->validateCookieRecord( $cookieRecord ) ) {
			// Most callers will probably already check this, but
			// doesn't hurt to be careful.
			return false;
		}
		$parts = explode( "-", $cookieRecord, 3 );
		$hash = $this->generateUserCookieRecord( $user->getName(), $parts[0], $parts[1] );
		return hash_equals( $hash, $cookieRecord );
	}

	/**
	 * Check if cookie is valid (Is not too old, has 3 fields)
	 *
	 * @param $cookieRecord Cookie record
	 * @return boolean True if valid
	 */
	private function validateCookieRecord( $cookieRecord ) {
		$parts = explode( "-", $cookieRecord, 3 );
		if ( count( $parts ) !== 3 || strlen( $parts[0] ) !== 4 ) {
			$this->log->warning( "Got cookie with invalid format",
				[
					'method' => __METHOD__,
					'cookieRecord' => $cookie
				]
			);
			return false;
		}
		if ( (int)$parts[0] < gmdate( 'Y' ) - 3 ) {
			// Record is too old. If user hasn't logged in from this
			// computer in two years, should probably not consider it trusted.
			return false;
		}
		return true;
	}

	/**
	 * Generate a single record for use in the previous login cookie
	 *
	 * The format is YYYY-SSSSSSS-HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH
	 * where Y is the year, S is a 32-bit salt, H is an sha1-hmac.
	 * Both S and H are base-36 encoded. The actual cookie consists
	 * of several of these records separated by a ".".
	 *
	 * When checking if a hash is valid, provide all three arguments.
	 * When generating a new hash, only use the first argument.
	 *
	 * @param $username String Username,
	 * @param $year int [Optional] Year. Default to current year
	 * @param $salt string [Optional] Salt (expected to be base-36 encoded)
	 * @return String A record for the cookie
	 */
	private function generateUserCookieRecord( $username, $year = false, $salt = false ) {
		if ( $year === false ) {
			$year = gmdate( 'Y' );
		}

		if ( $salt === false ) {
			$salt = Wikimedia\base_convert( MWCryptRand::generateHex( 8 ), 16, 36 );
		}

		// FIXME Maybe shorten, e.g. User only half the hash?
		$res = hash_hmac( 'sha1', $username . '|' . $year . $salt, $this->secret );
		if ( !is_string( $res ) ) {
			throw new UnexpectedValueException( "Hash failed" );
		}
		$encoded = $year . '-' . $salt . '-' . wfBaseConvert( $res, 16, 36 );
		return $encoded;
	}

	/**
	 * Get the cache key for the counter.
	 *
	 * @param $user User
	 * @param $type string 'known' or 'new'
	 * @return string The cache key
	 */
	private function getKey( User $user, $type ) {
		$userHash = Wikimedia\base_convert( sha1( $user->getName() ), 16, 36, 31 );
		return $this->cache->makeGlobalKey(
			'loginnotify', $type, $userHash
		);
	}

	/**
	 * Increment hit counters for a failed login from an unknown computer.
	 *
	 * If a sufficient number of hits have accumulated, send an echo notice.
	 *
	 * @param User $user
	 */
	private function incNewIP( User $user ) {
		$key = $this->getKey( $user, 'new' );
		$count = $this->checkAndIncKey(
			$key,
			$this->config->get( 'LoginNotifyAttemptsNewIP' ),
			$this->config->get( 'LoginNotifyExpiryNewIP' )
		);
		if ( $count ) {
			$this->sendNotice( $user, 'login-fail-new', $count );
		}
	}

	/*
	 * Increment hit counters for a failed login from a known computer.
	 *
	 * If a sufficient number of hits have accumulated, send an echo notice.
	 *
	 * @param User $user
	 */
	private function incKnownIP( User $user ) {
		$key = $this->getKey( $user, 'known' );
		$count = $this->checkAndIncKey(
			$key,
			$this->config->get( 'LoginNotifyAttemptsKnownIP' ),
			$this->config->get( 'LoginNotifyExpiryKnownIP' )
		);
		if ( $count ) {
			$this->sendNotice( $user, 'login-fail-known', $count );
		}
	}

	/**
	 * Send a notice about login attempts
	 *
	 * @param $user User The account in question
	 * @param $type String 'login-fail-new' or 'login-fail-known'
	 * @param $count int [Optional] How many failed attempts
	 */
	private function sendNotice( User $user, $type, $count = null ) {
		$extra = [ 'notifyAgent' => true ];
		if ( $count !== null ) {
			$extra['count'] = $count;
		}
		EchoEvent::create( [
			'type' => $type,
			'extra' => $extra,
			'agent' => $user,
		] );
	}

	/**
	 * Check if we've reached limit, and increment cache key.
	 *
	 * @param $key string cache key
	 * @param $max int interval of one to send notice
	 * @param $expiry int When to expire cache key.
	 * @return Bool|int false to not send notice, or number of hits
	 */
	private function checkAndIncKey( $key, $interval, $expiry ) {
		$cache = $this->cache;
		$cur = $cache->incr( $key );
		if ( !$cur ) {
			$cache->add( $key, 1, $expiry );
			$cur = 1;
		}
		if ( $cur % $interval === 0 ) {
			return $cur;
		}
		return false;
	}

	/**
	 * Clear attempt counter for user.
	 *
	 * When a user succesfully logs in, we start back from 0, as
	 * otherwise a mistake here and there will trigger the warning.
	 *
	 * @param $user User
	 */
	public function clearCounters( User $user ) {
		$cache = $this->cache;
		$keyKnown = $this->getKey( $user, 'known' );
		$keyNew = $this->getKey( $user, 'new' );

		$cache->delete( $keyKnown );
		$cache->delete( $keyNew );
	}

	/**
	 * On login failure, record failure and maybe send notice
	 *
	 * @param $user User The user whose account was attempted to log into
	 */
	public function recordFailure( User $user ) {
		$fromKnownIP = $this->isFromKnownIP( $user );
		if ( $fromKnownIP ) {
			$this->incKnownIP( $user );
		} else {
			$this->incNewIP( $user );
		}
	}

	/**
	 * Send a notice on successful login if not known ip
	 *
	 * @param $user User Account in question
	 */
	public function sendSuccessNotice( User $user ) {
		if ( $this->config->get( 'LoginNotifyEnableOnSuccess' )
			&& !$this->isFromKnownIP( $user )
		) {
			$this->sendNotice( $user, 'login-success' );
		}
	}
}
