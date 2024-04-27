<?php
if (!defined('MEDIAWIKI')) {
	exit;
}

$xvWikis = [
	'meta' => 'meta.w.xvnet.eu.org',
	'xvnet' => 'w.xvnet.eu.org',
];

if (defined('MW_DB')) {
	$xvWikiID = MW_DB;
	$xvMaintScript = true;
} else if ($_SERVER['MW_WIKI']) {
	$xvWikiID = $_SERVER['MW_WIKI'];
	$xvMaintScript = false;
} else {
	die('Unknown wiki.');
}

$xvHttpHost = $_SERVER['HTTP_HOST'];

$xvLoadExtensions = [];
$xvLoadSkins = [];

require_once ('/srv/secrets/mw/Secrets.php');
require_once (dirname(__FILE__) . '/GlobalSettings.php');
require_once (dirname(__FILE__) . '/LocalSettings.' . $xvWikiID . '.php');

wfLoadExtensions($xvLoadExtensions);
wfLoadSkins($xvLoadSkins);
