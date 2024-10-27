<?php

namespace MediaWiki\Extension\XensTweaks;

use Html;
use MediaWiki\MediaWikiServices;
use OutputPage;
use Skin;
use Title;
use WikiMap;

/**
 * Hooks for XensTweaks extension
 *
 * @file
 * @ingroup Extensions
 */
class XensTweaksHooks {
	/**
	 * Add some links at the bottom of pages
	 *
	 * @param Skin $skin
	 * @param string $key
	 * @param array &$footerLinks
	 */
	public static function onSkinAddFooterLinks( Skin $skin, string $key, array &$footerLinks ) {
		global $wgXensTweaksAddFooterLinks;

		if ( $wgXensTweaksAddFooterLinks && $key === 'places' ) {
			$footerLinks['tos'] = Html::element(
				'a',
				[
					'href' => Skin::makeInternalOrExternalUrl(
						$skin->msg( 'xens-tos-url' )->inContentLanguage()->text()
					),
				],
				$skin->msg( 'xens-tos' )->text()
			);
		}
	}

	/**
	 * Protect Xens Wikis system messages from being edited by those that do not have
	 * the "editinterfacesite" right. This is because system messages that are prefixed
	 * with "xens" are probably there for a legal reason or to ensure consistency
	 * across the site.
	 *
	 * @return bool
	 */
	public static function ongetUserPermissionsErrors( $title, $user, $action, &$result ) {
		global $wgXensTweaksProtectSiteInterface;

		if ( $wgXensTweaksProtectSiteInterface
			&& $action !== 'read'
			&& $title->inNamespace( NS_MEDIAWIKI )
			&& strpos( lcfirst( $title->getDBKey() ), 'xens-' ) === 0
			&& !$user->isAllowed( 'editinterfacesite' )
		) {
			$result = 'xens-siteinterface';
			return false;
		}

		return true;
	}

	/**
	 * Implement theming and add structured data for the Google Sitelinks search box.
	 */
	public static function onBeforePageDisplay( OutputPage &$out, Skin &$skin ) {
		global $wgSitename;
		global $wgXensTweaksEnableSearchboxMetadata, $wgArticlePath, $wgCanonicalServer;

		$title = $out->getTitle();
		if ( $title->isMainPage() ) {
			/* Open Graph protocol */
			$out->addMeta( 'og:title', $wgSitename );
			$out->addMeta( 'og:type', 'website' );

			/* Structured data for the Google Sitelinks search box. */
			if ( $wgXensTweaksEnableSearchboxMetadata ) {
				$targetUrl = $wgCanonicalServer . str_replace( '$1', 'Special:Search', $wgArticlePath );
				$targetUrl = wfAppendQuery( $targetUrl, 'search={search_term_string}' );
				$structuredData = [
					'@context'        => 'http://schema.org',
					'@type'           => 'WebSite',
					'url'             => $wgCanonicalServer,
					'potentialAction' => [
						'@type'       => 'SearchAction',
						'target'      => $targetUrl,
						'query-input' => 'required name=search_term_string',
					],
				];
				$out->addHeadItem( 'StructuredData', '<script type="application/ld+json">' . json_encode( $structuredData ) . '</script>' );
			}
		} else {
			/* Open Graph protocol */
			$out->addMeta( 'og:site_name', $wgSitename );
			$out->addMeta( 'og:title', $title->getPrefixedText() );
			$out->addMeta( 'og:type', 'article' );
		}
		/* Open Graph protocol */
		$out->addMeta( 'og:url', $title->getFullURL() );

		// Baidu site verification
		global $wgXensTweaksMetaSiteVerifications;
		if ( $title->isMainPage() ) {
			if ( isset( $wgXensTweaksMetaSiteVerifications['360-site'] ) ) {
				$out->addMeta( '360-site-verification', htmlspecialchars( $wgXensTweaksMetaSiteVerifications['360-site'] ) );
			}
			if ( isset( $wgXensTweaksMetaSiteVerifications['baidu'] ) ) {
				$out->addMeta( 'baidu-site-verification', htmlspecialchars( $wgXensTweaksMetaSiteVerifications['baidu'] ) );
			}
		}
	}

	// Cache OpenSearch for 600 seconds. (10 minutes)
	public static function onOpenSearchUrls( &$urls ) {
		foreach ( $urls as &$url ) {
			if ( in_array( $url['type'], [ 'application/x-suggestions+json', 'application/x-suggestions+xml' ] ) ) {
				$url['template'] = wfAppendQuery( $url['template'], [ 'maxage' => 600, 'smaxage' => 600, 'uselang' => 'content' ] );
			}
		}
	}

	/**
	 * Prevent infinite looping of main page requests with cache parameters.
	 */
	public static function onTestCanonicalRedirect( $request, $title, $output ) {
		global $wgScriptPath;
		if ( $title->isMainPage() && str_starts_with( $request->getRequestURL(), $wgScriptPath . '/?' ) ) {
			return false;
		}
	}

	/**
	 * Use Short URL always, even for queries.
	 * Additionally apply it to the main page
	 * because $wgMainPageIsDomainRoot doesn't apply to the internal URL, which is used for purging.
	 */
	public static function onGetLocalURLInternal( $title, &$url, $query ) {
		global $wgArticlePath, $wgScript, $wgMainPageIsDomainRoot, $wgScriptPath;
		$dbkey = wfUrlencode( $title->getPrefixedDBkey() );
		if ( $wgMainPageIsDomainRoot && $title->isMainPage() ) {
			$url = wfAppendQuery( $wgScriptPath . '/', $query );
		} elseif ( $url == "{$wgScript}?title={$dbkey}&{$query}" ) {
			$url = wfAppendQuery( str_replace( '$1', $dbkey, $wgArticlePath ), $query );
		}
	}

	/**
	 * Add purging for global robots.txt, well-known URLs, and hashless images.
	 */
	public static function onTitleSquidURLs( Title $title, array &$urls ) {
		global $wgCanonicalServer, $wgXensTweaksCentralDB, $wgDBname;
		$dbkey = $title->getPrefixedDBKey();
		// MediaWiki:Robots.txt on metawiki is global.
		if ( $wgDBname === $wgXensTweaksCentralDB && $dbkey === 'MediaWiki:Robots.txt' ) {
			// Purge each wiki's /robots.txt route.
			foreach ( WikiMap::getCanonicalServerInfoForAllWikis() as $serverInfo ) {
				$urls[] = $serverInfo['url'] . '/robots.txt';
			}
		} elseif ( $dbkey === 'File:Apple-touch-icon.png' ) {
			$urls[] = $wgCanonicalServer . '/apple-touch-icon.png';
		} elseif ( $dbkey === 'File:Favicon.ico' ) {
			$urls[] = $wgCanonicalServer . '/favicon.ico';
		} elseif ( $title->getNamespace() == NS_FILE ) {
			$file = MediaWikiServices::getInstance()->getRepoGroup()->getLocalRepo()->newFile( $title );
			if ( $file ) {
				$urls[] = strtok( $file->getUrl(), '?' );
			}
		}
	}
}
