<?php
$wgSitename = "xtex's K";
$wgMetaNamespace = "K";

$xvTesting = true;
$xvCentralized = true;
$xvUseSelfHostedImages = true;

$xvUseLockdown = true;

require_once "$xvConfigDirectory/common/LoadExtensions.php";

$wgLanguageCode = 'zh';
$wgLocaltimezone = 'Asia/Shanghai';

$wgRightsUrl = "https://creativecommons.org/licenses/by-sa/4.0/";
$wgRightsText = "CC BY-SA 4.0";
$wgRightsIcon = "$wgResourceBasePath/resources/assets/licenses/cc-by-sa.png";

xvLoadSkin('Lakeus');
$wgDefaultSkin = $wgDefaultMobileSkin = 'lakeus';

$xvUseEmailConfirmed = true;

// User rights
$xvRequireEmailConfirmedToEdit = true;
$wgGroupPermissions['l-accessor'] = ['read', 'autopatrol'];
$wgGroupPermissions['m-accessor'] = ['read', 'autopatrol'];
$wgAddGroups['m-accessor'] = ['l-accessor'];
$wgGroupsAddToSelf['m-accessor'] = ['l-accessor'];

// Namespaces
const NS_L = 3000;
const NS_L_TALK = 3001;
$wgExtraNamespaces[NS_L] = 'L';
$wgExtraNamespaces[NS_L_TALK] = 'L讨论';
$wgContentNamespaces[] = NS_L;
xvSetAssocTrues('wgNamespacesWithSubpages', [NS_L, NS_L_TALK]);
xvMergeInto('wgNonincludableNamespaces', [NS_L, NS_L_TALK]);
$wgVisualEditorAvailableNamespaces['L'] = true;

const NS_M = 3002;
const NS_M_TALK = 3003;
$wgExtraNamespaces[NS_L] = 'M';
$wgExtraNamespaces[NS_L_TALK] = 'M讨论';
$wgContentNamespaces[] = NS_M;
xvSetAssocTrues('wgNamespacesWithSubpages', [NS_M, NS_L_TALK]);
xvMergeInto('wgNonincludableNamespaces', [NS_M, NS_L_TALK]);
$wgVisualEditorAvailableNamespaces['M'] = true;

// Lockdown
$wgNamespacePermissionLockdown[NS_L]['read'] = ['l-accessor'];
$wgNamespacePermissionLockdown[NS_L]['edit'] = ['l-accessor'];
$wgNamespacePermissionLockdown[NS_M]['read'] = ['m-accessor'];
$wgNamespacePermissionLockdown[NS_M]['edit'] = ['m-accessor'];

require_once "$xvConfigDirectory/common/GlobalSettings.php";
