<?php
$wgSitename = "Xensor V Wiki";
$wgMetaNamespace = "XvWiki";

$xvTesting = true;

$wgLanguageCode = 'en';
$wgLocaltimezone = 'UTC';

$wgRightsUrl = "https://creativecommons.org/licenses/by-sa/4.0/";
$wgRightsText = "Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)";
$wgRightsIcon = "$wgResourceBasePath/resources/assets/licenses/cc-by-sa.png";

$xvUseGlobalSkins = false;
xvLoadSkin('Lakeus');
$wgDefaultSkin = $wgDefaultMobileSkin = 'lakeus';

$wgLocalInterwikis[] = 'xvn';

$wgAutopromote['emailconfirmed'] = APCOND_EMAILCONFIRMED;
$wgImplicitGroups[] = 'emailconfirmed';
xvRemovePermission('edit', ['*', 'user']);
$wgGroupPermissions['emailconfirmed']['edit'] = true;

xvLoadExtension('Cargo');

require_once "$xvConfigDirectory/common/GlobalSettings.php";
