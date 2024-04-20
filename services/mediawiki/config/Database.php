<?php

$wgLocalDatabases = $wgConf->wikis = array_map(function ($n) {
	return 'wiki' . $n;
}, $wikis);

$wgDBtype = 'mysql';
$wgDBname = 'wiki' . $wikiID;
$wgDBserver = 'opilio.s.xvnet0.eu.org';
$wgDBport = 3307;
$wgDBuser = 'mediawiki';
$wgDBadminuser = 'mediawikiadmin';

$wgCompressRevisions = true;
