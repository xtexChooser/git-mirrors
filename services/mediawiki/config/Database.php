<?php

$wgLocalDatabases = $wgConf->wikis = array_map(function ($n) {
	'wiki' . $n;
}, $wikis);

$wgDBtype = 'postgres';
$wgDBname = 'wiki' . $wikiID;
$wgDBuser = 'mediawiki';
$wgDBadminuser = 'mediawikiadmin';