<?php
$wgSitename = "Xens Meta";
$wgMetaNamespace = "Meta";

require_once (dirname(__FILE__) . '/SharedTables.php');

$wgRightsUrl = "https://creativecommons.org/licenses/by-sa/4.0/";
$wgRightsText = "Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)";
$wgRightsIcon = "$wgResourceBasePath/resources/assets/licenses/cc-by-sa.png";

$xvLoadSkins[] = 'Lakeus';
$wgDefaultSkin = 'lakeus';

$wgLocalInterwikis[] = 'meta';
