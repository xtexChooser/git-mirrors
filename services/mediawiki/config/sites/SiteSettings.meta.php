<?php
$wgSitename = "Xens Meta";
$wgMetaNamespace = "Meta";

require_once "$xvConfigDirectory/common/LoadExtensions.php";

$wgLanguageCode = 'en';
$wgLocaltimezone = 'UTC';

$wgRightsUrl = "https://creativecommons.org/licenses/by-sa/4.0/";
$wgRightsText = "Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)";
$wgRightsIcon = "$wgResourceBasePath/resources/assets/licenses/cc-by-sa.png";

xvLoadSkin('Lakeus');
$wgDefaultSkin = $wgDefaultMobileSkin = 'lakeus';

$wgLocalInterwikis[] = 'meta';

require_once "$xvConfigDirectory/common/GlobalSettings.php";
