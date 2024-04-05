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


require_once ('/srv/secrets/mw/Secrets.php');
require_once (dirname(__FILE__) . '/GlobalSettings.php');
require_once (dirname(__FILE__) . '/LocalSettings.' . $wikiID . '.php');
