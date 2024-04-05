<?php
if (!defined('MEDIAWIKI')) {
	exit;
}

$wikis = [
	'meta',
	'xvnet',
];

if (defined('MW_DB')) {
	$wikiID = MW_DB;
} else if ($_SERVER['MW_WIKI']) {
	$wikiID = $_SERVER['MW_WIKI'];
} else {
	die('Unknown wiki.');
}

$wgLocalDatabases = $wgConf->wikis = array_map(function ($n) {
	'wiki' . $n;
}, $wikis);
$wgDBname = 'wiki' . $wikiID;

require_once (dirname(__FILE__) . '/GlobalSettings.php');
require_once (dirname(__FILE__) . '/LocalSettings.' . $wikiID . '.php');
