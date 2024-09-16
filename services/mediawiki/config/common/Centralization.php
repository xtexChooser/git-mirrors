<?php

// Shared Tables
$wgSharedDB = 'wiki' . $xvCentralWiki;
$wgSharedPrefix = '';

$wgSharedTables = [
	'user',
	'user_autocreate_serial',
	'actor',
	'sites',
];

// Shared Cookies
if (str_ends_with($xvHttpHost, 'w.xvnet.eu.org'))
	$wgCookieDomain = '.w.xvnet.eu.org';
else if (str_ends_with($xvHttpHost, 'w.xvnet0.eu.org'))
	$wgCookieDomain = '.w.xvnet0.eu.org';

// Resource Loaders
$wgResourceLoaderSources['metawiki'] = array(
	'apiScript' => 'https://meta.w.xvnet.eu.org/api.php',
	'loadScript' => 'https://meta.w.xvnet.eu.org/load.php',
);

// GlobalBlocking
if ($xvUseGlobalBlocking) {
	wfLoadExtension('GlobalBlocking');
	$wgDatabaseVirtualDomains['virtual-globalblocking'] = 'wikimeta';
	$wgGlobalBlockingBlockXFF = true;
	$wgGlobalBlockRemoteReasonUrl = $wgResourceLoaderSources['metawiki']['apiScript'];
}

// GlobalUserrights
if ($xvUseGlobalUserrights) {
	wfLoadExtension('GlobalUserrights');
	$wgSharedTables[] = 'global_user_groups';
}

// GlobalCssJs
if ($xvUseGlobalCssJs) {
	wfLoadExtension('GlobalCssJs');
	$wgUseGlobalSiteCssJs = true;
	$wgGlobalCssJsConfig = [
		'wiki' => $wgSharedDB,
		'source' => $xvWikiID != 'meta' ? 'metawiki' : 'local',
		'baseurl' => 'https://meta.w.xvnet.eu.org/w'
	];
}

// GlobalUserPage
if ($xvUseGlobalUserPage) {
	$xvLoadExtensions[] = 'GlobalUserPage';
	$wgGlobalUserPageAPIUrl = $wgResourceLoaderSources['metawiki']['apiScript'];
	$wgGlobalUserPageDBname = $wgSharedDB;
}

// GlobalPreferences
if ($xvUseGlobalPreferences) {
	wfLoadExtension('GlobalPreferences');
}

// OAuth
$wgMWOAuthCentralWiki = $wgSharedDB;

// ThrottleOverride
$wgThrottleOverrideCentralWiki = $wgSharedDB;

// AntiSpoof
if (xvIsExtensionLoaded('AntiSpoof')) {
	$wgSharedTables[] = 'spoofuser';
}
