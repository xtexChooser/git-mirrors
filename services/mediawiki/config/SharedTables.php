<?php

$wgSharedDB = 'wikimeta';
$wgSharedPrefix = '';

$wgSharedTables[] = 'actor';
$wgSharedTables[] = 'spoofuser';
if (str_ends_with($_SERVER['HTTP_HOST'], 'w.xvnet.eu.org')) {
	$wgCookieDomain = '.w.xvnet.eu.org';
} else if (str_ends_with($_SERVER['HTTP_HOST'], 'w.xvnet0.eu.org')) {
	$wgCookieDomain = '.w.xvnet0.eu.org';
}

// Global Blocking
$xvLoadExtensions[] = 'GlobalBlocking';
$wgDatabaseVirtualDomains['virtual-globalblocking'] = $wgSharedDB;
$wgGlobalBlockingBlockXFF = true;

// Global User Groups
$xvLoadExtensions[] = 'GlobalUserrights';
$wgSharedTables[] = 'global_user_groups';

// Global CSS/JS
$xvLoadExtensions[] = 'GlobalCssJs';
if ($wikiID != 'meta') {
	$wgGlobalCssJsConfig = [
		'wiki' => 'wikimeta',
		'source' => 'metawiki',
	];
} else {
	$wgGlobalCssJsConfig = [
		'wiki' => $wgDBname,
		'source' => 'local',
	];
}
$wgResourceLoaderSources['metawiki'] = array(
	'apiScript' => 'https://meta.w.xvnet.eu.org/api.php',
	'loadScript' => 'https://meta.w.xvnet.eu.org/load.php',
);

// Global User Page
if ($wikiID != 'meta') {
	$xvLoadExtensions[] = 'GlobalUserPage';
	$wgGlobalUserPageAPIUrl = $wgResourceLoaderSources['metawiki']['apiScript'];
	$wgGlobalUserPageDBname = $wgSharedDB;
}
