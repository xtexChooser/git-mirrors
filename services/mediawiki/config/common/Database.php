<?php

$wgLocalDatabases = $wgConf->wikis = array_map(function ($n) {
	return 'wiki' . $n;
}, array_keys($xvWikis));

$wgDBtype = 'mysql';
$wgDBname = 'wiki' . $xvWikiID;
$wgDBserver = 'opilio.s.xvnet0.eu.org:3307';
$wgDBuser = 'mediawiki';
$wgDBadminuser = 'mediawikiadmin';

$wgCompressRevisions = true;

// Cargo
$wgCargoDBname = 'wikicargo' . $xvWikiID;
$wgCargoDBtype = $wgDBtype;
$wgCargoDBserver = $wgDBserver;
$wgCargoDBuser = 'mediawikicargo';
