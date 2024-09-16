<?php
if (!defined('MEDIAWIKI')) {
	die('Not an entry point.');
}

if (defined('MW_DB')) {
	$xvWikiID = MW_DB;
	$xvMaintScript = true;
} else if ($_SERVER['MW_WIKI'] ?: false) {
	$xvWikiID = $_SERVER['MW_WIKI'];
	$xvMaintScript = false;
} else {
	die('Unknown wiki.');
}

$xvWikis = [
	'meta' => 'meta.w.xvnet.eu.org',
	'xvnet' => 'w.xvnet.eu.org',
];

$xvDebug = false;
if ($_SERVER['MW_DEBUG'] ?: false) {
	$xvDebug = true;
} else if ($_SERVER['HTTP_X_XENS_WIKIS_DEBUG'] ?: false) {
	$xvDebug = true;
}

if ($xvDebug) {
	$wgDevelopmentWarnings = true;
	ini_set('display_errors', true);
	header('X-Xens-Wikis-Debug: true');
}

setlocale(LC_ALL, 'en_US.UTF-8');

if (PHP_SAPI === 'cli') {
	$wgRequestTimeLimit = 0;
} elseif ($xvMaintScript) {
	$wgRequestTimeLimit = 86400;
} elseif ($_SERVER['REQUEST_METHOD'] === 'POST') {
	$wgRequestTimeLimit = 200;
} else {
	$wgRequestTimeLimit = 60;
}

$xvServerName = $xvWikis[$xvWikiID];
$xvHttpHost = $_SERVER['HTTP_HOST'] ?? $xvServerName;

$xvLoadExtensions = [];
$xvLoadSkins = [];

function xvLoadConfig($file) {
	require_once('/etc/mediawiki/' . $file);
}

require_once('/srv/secrets/mw/Secrets.php');
xvLoadConfig('GlobalSettings.php');
xvLoadConfig('LocalSettings.' . $xvWikiID . '.php');

wfLoadExtensions($xvLoadExtensions);
wfLoadSkins($xvLoadSkins);
