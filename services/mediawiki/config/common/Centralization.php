<?php

// Shared Tables
$wgSharedDB = "wiki$xvCentralWiki";
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
$wgResourceLoaderSources['metawiki'] = [
	'apiScript' => 'https://meta.w.xvnet.eu.org/api.php',
	'loadScript' => 'https://meta.w.xvnet.eu.org/load.php',
];

// GlobalBlocking
if ($xvUseGlobalBlocking) {
	xvLoadExtension('GlobalBlocking');
	$wgDatabaseVirtualDomains['virtual-globalblocking'] = 'wikimeta';
	$wgGlobalBlockingBlockXFF = true;
	$wgGlobalBlockRemoteReasonUrl = $wgResourceLoaderSources['metawiki']['apiScript'];
}

// GlobalUserrights
if ($xvUseGlobalUserrights) {
	xvLoadExtension('GlobalUserrights');
	$wgSharedTables[] = 'global_user_groups';
}

// GlobalCssJs
if ($xvUseGlobalCssJs) {
	xvLoadExtension('GlobalCssJs');
	$wgUseGlobalSiteCssJs = true;
	$wgGlobalCssJsConfig = [
		'wiki' => $wgSharedDB,
		'source' => $xvWikiID != 'meta' ? 'metawiki' : 'local',
		'baseurl' => 'https://meta.w.xvnet.eu.org/w'
	];
}

// GlobalUserPage
if ($xvUseGlobalUserPage) {
	xvLoadExtension('GlobalUserPage');
	$wgGlobalUserPageAPIUrl = $wgResourceLoaderSources['metawiki']['apiScript'];
	$wgGlobalUserPageDBname = $wgSharedDB;
}

// GlobalPreferences
if ($xvUseGlobalPreferences) {
	xvLoadExtension('GlobalPreferences');
}

// OAuth
$wgMWOAuthCentralWiki = $wgSharedDB;

// ThrottleOverride
$wgThrottleOverrideCentralWiki = $wgSharedDB;

// AntiSpoof
if (xvIsExtensionLoaded('AntiSpoof')) {
	$wgSharedTables[] = 'spoofuser';
}
