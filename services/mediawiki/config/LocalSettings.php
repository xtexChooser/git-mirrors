<?php
if (!defined('MEDIAWIKI')) {
	exit;
}

$wikis = [
	'wikimeta',
	'wikixvnet',
];

if (defined('MW_DB')) {
	$wikiID = MW_DB;
} else {
	$wikiID = $_SERVER['MW_DB'] ?? null;
	if (!$wikiID) {
		die('Unknown wiki.');
	}
}

$wgLocalDatabases = $wgConf->wikis = $wikis;
$wgDBname = $wikiID;
$wgDBuser = 'mediawiki';
