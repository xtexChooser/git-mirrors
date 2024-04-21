<?php

$wgLocalDatabases = $wgConf->wikis = array_map(function ($n) {
	return 'wiki' . $n;
}, array_values($xvWikis));

$wgDBtype = 'mysql';
$wgDBname = 'wiki' . $xvWikiID;
$wgDBserver = 'opilio.s.xvnet0.eu.org:3307';
// $wgDBport = 3307;
$wgDBuser = 'mediawiki';
$wgDBadminuser = 'mediawikiadmin';

$wgCompressRevisions = true;
