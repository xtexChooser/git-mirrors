<?php

$wgLocalDatabases = [];
foreach ($xvWikis as $id => $config) {
	if ($config['family'] == $xvWikiFamily) {
		$wgLocalDatabases[] = 'wiki' . $id;
	}
}
$wgConf->wikis = $wgLocalDatabases;

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
