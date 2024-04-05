<?php

$wgLocalDatabases = $wgConf->wikis = array_map(function ($n) {
	return 'wiki' . $n;
}, $wikis);

$wgDBtype = 'postgres';
$wgDBname = 'wiki' . $wikiID;
$wgDBserver = 'opilio.s.xvnet0.eu.org';
$wgDBport = 5433;
$wgDBuser = 'mediawiki';
$wgDBadminuser = 'mediawikiadmin';

$wgCompressRevisions = true;
