<?php

namespace MediaWiki\Extension\GloopTweaks;

use EventRelayer;

/**
 * EventRelayer to perform Cloudflare purging.
 * Note: This performs purges directly, so if purging fails for any reason, the purges are lost.
 *
 */
class GloopEventRelayer extends EventRelayer {
	// Cloudflare limits purge_cache API to 30 URLs per request.
	private const MAX_URLS_PER_REQUEST = 30;

	public function doNotify( $channel, array $events ) {
		// This EventRelayer is for CDN URL purges only.
		if ( $channel != 'cdn-url-purges' ) {
			return false;
		}

		// Extract the URLs to purge from the 'cdn-url-purges' events.
		$urls = [];
		foreach ( $events as $event ) {
			// File purges include only hostname, so the URL must be expanded.
			$urls[] = wfExpandUrl( $event['url'], PROTO_INTERNAL );
		}

		// Purge the URLs from Cloudflare.
		if ( count( $urls ) > 0 ) {
			// Deduplicate URLs.
			$urls = array_unique( $urls );

			wfDebugLog( 'purges_cf', __METHOD__ . ': ' . implode( ' ', $urls ) );
			self::CloudflarePurge( $urls );
		}

		return true;
	}

	/**
	* Send Cloudflare purge requests.
	*
	* @param string[] $urls Array of URLs to purge.
	*/
	private static function CloudflarePurge( array $urls ) {
		global $wgGloopTweaksCFToken, $wgGloopTweaksCFZone;

		// Break the purge requests into chunks sized to Cloudflare's per-request URL limit.
		$chunks = array_chunk( $urls, self::MAX_URLS_PER_REQUEST );

		// Prepare cURL
		$ch = curl_init();
		curl_setopt( $ch, CURLOPT_PROTOCOLS, CURLPROTO_HTTP | CURLPROTO_HTTPS );
		curl_setopt( $ch, CURLOPT_HTTPHEADER, [
			'Authorization: Bearer ' . $wgGloopTweaksCFToken,
			'Content-Type: application/json',
		] );
		curl_setopt( $ch, CURLOPT_RETURNTRANSFER, true );
		curl_setopt( $ch, CURLOPT_CONNECTTIMEOUT, 10 );
		curl_setopt( $ch, CURLOPT_TIMEOUT, 10 );
		curl_setopt( $ch, CURLOPT_URL, 'https://api.cloudflare.com/client/v4/zones/' . $wgGloopTweaksCFZone . '/purge_cache' );

		// Perform the purge requests a chunk at a time.
		foreach ( $chunks as $chunk ) {
			curl_setopt( $ch, CURLOPT_POSTFIELDS, '{"files":' . json_encode( $chunk, JSON_UNESCAPED_SLASHES ) . '}' );
			curl_exec( $ch );
		}
		curl_close( $ch );
	}
}
