<?php
// This file is intended to be symlinked into $IP.

use MediaWiki\MediaWikiServices;
use MediaWiki\Revision\SlotRecord;

define( 'MW_NO_SESSION', 1 );
define( 'MW_ENTRY_POINT', 'robots' );

require dirname( $_SERVER['SCRIPT_FILENAME'] ) . '/includes/WebStart.php';

wfRobotsMain();

function wfRobotsMain() {
	global $wgXensTweaksCentralDB, $wgCanonicalServer, $wgXensTweaksNoRobots, $wgNamespaceRobotPolicies;

	if ( $wgXensTweaksNoRobots ) {
		header( 'Cache-Control: max-age=300, must-revalidate, s-maxage=300, revalidate-while-stale=300' );
		header( 'Content-Type: text/plain; charset=utf-8' );
		echo "User-agent: *\nDisallow: /";
		return;
	}

	$services = MediaWikiServices::getInstance();

	$title = $services->getTitleParser()->parseTitle( 'MediaWiki:Robots.txt' );
	$store = $services->getRevisionStoreFactory()->getRevisionStore( $wgXensTweaksCentralDB );
	$rev = $store->getRevisionByTitle( $title );
	$content = $rev ? $rev->getContent( SlotRecord::MAIN ) : null;
	$lastModified = $rev ? $rev->getTimestamp() : null;
	$text = ( $content instanceof TextContent ) ? $content->getText() : '';

	// Disallow noindexed namespaces in robots.txt as well.
	$contLang = $services->getContentLanguage();
	$langConverter = $services->getLanguageConverterFactory()->getLanguageConverter( $contLang );
	$namespaceInfo = $services->getNamespaceInfo();
	$namespaces = [];

	// NS_SPECIAL is hardcoded as noindex, but not normally in $wgNamespaceRobotPolicies.
	$wgNamespaceRobotPolicies[NS_SPECIAL] = 'noindex';

	foreach ( $wgNamespaceRobotPolicies as $ns => $policy ) {
		if ( str_contains( $policy, 'noindex' ) ) {
			$name = $contLang->getNsText( $ns );
			if ( $name !== '' ) {
				$namespaces[] = $name;
			}
		}
	}

	$disallowText = 'User-Agent: *';
	foreach ( $namespaces as $ns ) {
		$lcns = strtolower( $ns );
		$disallowText .= <<<DISALLOW

		Disallow: /w/$ns:
		Disallow: /w/$ns%3A
		Disallow: /w/$lcns:
		Disallow: /*?title=$ns:
		Disallow: /*?title=$ns%3A
		Disallow: /*?*&title=$ns:
		Disallow: /*?*&title=$ns%3A
		DISALLOW;
	}
	if ( $text ) {
		$text = str_replace( 'User-Agent: *', $disallowText, $text );
	} else {
		$text = $disallowText;
	}

	header( 'Cache-Control: max-age=300, must-revalidate, s-maxage=3600, revalidate-while-stale=300' );
	header( 'Content-Type: text/plain; charset=utf-8' );

	if ( $lastModified ) {
		header( 'Last-Modified: ' . wfTimestamp( TS_RFC2822, $lastModified ) );
	}

	$sitemap = "Sitemap: $wgCanonicalServer/images/sitemaps/index.xml";
	if ( $text ) {
		echo $text . "\n\n" . $sitemap;
	} else {
		echo $sitemap;
	}
}
