<?php

$xvSharedWiki = 'meta';
$wgSharedDB = 'wiki' . $xvSharedWiki;
$wgSharedPrefix = '';

$wgSharedTables = [
	'user',
	'user_autocreate_serial',
	'actor',
	'spoofuser',
	'sites',
];
if (str_ends_with($xvHttpHost, 'w.xvnet.eu.org')) {
	$wgCookieDomain = '.w.xvnet.eu.org';
} else if (str_ends_with($xvHttpHost, 'w.xvnet0.eu.org')) {
	$wgCookieDomain = '.w.xvnet0.eu.org';
}

$wgResourceLoaderSources['metawiki'] = array(
	'apiScript' => 'https://meta.w.xvnet.eu.org/api.php',
	'loadScript' => 'https://meta.w.xvnet.eu.org/load.php',
);

// Global Blocking
$xvLoadExtensions[] = 'GlobalBlocking';
$wgDatabaseVirtualDomains['virtual-globalblocking'] = 'wikimeta';
$wgGlobalBlockingBlockXFF = true;
$wgGlobalBlockRemoteReasonUrl = $wgResourceLoaderSources['metawiki']['apiScript'];

// Global User Groups
$xvLoadExtensions[] = 'GlobalUserrights';
$wgSharedTables[] = 'global_user_groups';

// Global CSS/JS
$xvLoadExtensions[] = 'GlobalCssJs';
$wgUseGlobalSiteCssJs = true;
$wgGlobalCssJsConfig = [
	'wiki' => $wgSharedDB,
	'source' => $xvWikiID != 'meta' ? 'metawiki' : 'local',
	'baseurl' => 'https://meta.w.xvnet.eu.org/w'
];

// Global User Page
$xvLoadExtensions[] = 'GlobalUserPage';
$wgGlobalUserPageAPIUrl = $wgResourceLoaderSources['metawiki']['apiScript'];
$wgGlobalUserPageDBname = $wgSharedDB;

// Global Preferences
$xvLoadExtensions[] = 'GlobalPreferences';

// OAuth
$wgMWOAuthCentralWiki = $wgSharedDB;
