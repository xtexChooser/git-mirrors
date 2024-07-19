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

// Global Blocking
$xvLoadExtensions[] = 'GlobalBlocking';
$wgDatabaseVirtualDomains['virtual-globalblocking'] = 'wikimeta';
$wgGlobalBlockingDatabase = 'wikimeta'; // TODO: Remove after 1.42 releasing
$wgGlobalBlockingBlockXFF = true;

// Global User Groups
$xvLoadExtensions[] = 'GlobalUserrights';
$wgSharedTables[] = 'global_user_groups';

// Global CSS/JS
$xvLoadExtensions[] = 'GlobalCssJs';
$wgUseGlobalSiteCssJs = true;
$wgResourceLoaderSources['metawiki'] = array(
	'apiScript' => 'https://meta.w.xvnet.eu.org/api.php',
	'loadScript' => 'https://meta.w.xvnet.eu.org/load.php',
);
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
