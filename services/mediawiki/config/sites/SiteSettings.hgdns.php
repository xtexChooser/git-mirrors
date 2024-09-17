<?php
$wgSitename = "幻光电脑社";
$wgMetaNamespace = "Project";
$wgLocalInterwikis[] = 'hgdns';
$xvCentralized = false;

// FIXME: Disable lockdown after configured
$xvEmergSecLockdown = true;

// Localisation
$wgLanguageCode = 'zh-hans';
xvMergeInto('wgHiddenPrefs', ['language', 'variant', 'noconvertlink']);
$wgLocaltimezone = 'Asia/Shanghai';
xvMergeInto('wgHiddenPrefs', ['date', 'timecorrection']);
$wgDefaultUserOptions['date'] = 'ISO 8601';

// Copyrights
$wgRightsUrl = "https://creativecommons.org/publicdomain/zero/1.0/legalcode.txt";
$wgRightsText = "CC0 1.0公有领域";
$wgRightsIcon = "$wgResourceBasePath/resources/assets/licenses/cc-0.png";

// Default User Options
$wgDefaultUserOptions['watchcreations'] = 0;
$wgDefaultUserOptions['watchuploads'] = 0;
$wgDefaultUserOptions['watchdefault'] = 0;

// Skin
xvLoadSkin('Citizen');
$wgDefaultSkin = $wgDefaultMobileSkin = 'Citizen';
$wgCitizenShowPageTools = 'login';

// User Rights
xvRemovePermission('edit', ['*']);
xvRemovePermission('createaccount', groups: ['*', 'user']);
xvGrantPermissionsTo('user', [
	'read',
	'edit',
	'delete-redirect',
	'editsemiprotected',
]);
xvGrantPermissionsTo('sysop', ['createaccount']);
xvGrantPermissionsTo('staff', ['createaccount']);
$wgAutoConfirmAge = 0;
$wgAutoConfirmCount = 0;

// Misc
$xvUseCaptcha = false;
$wgUseSharedUploads = false;

// Namespaces
const NS_MEMBER = 3000;
const NS_MEMBER_TALK = 3001;
$wgExtraNamespaces[NS_MEMBER] = 'Member';
$wgExtraNamespaces[NS_MEMBER_TALK] = 'Member_talk';
$wgContentNamespaces[] = NS_MEMBER;
$wgNamespaceAliases['M'] = NS_MEMBER;
$wgNamespaceAliases['MT'] = NS_MEMBER_TALK;
xvSetAssocTrues('wgNamespacesWithSubpages', [NS_MEMBER, NS_MEMBER_TALK]);
xvMergeInto('wgNonincludableNamespaces', [NS_MEMBER, NS_MEMBER_TALK]);

const NS_STAFF = 3002;
const NS_STAFF_TALK = 3003;
$wgExtraNamespaces[NS_STAFF] = 'Staff';
$wgExtraNamespaces[NS_STAFF_TALK] = 'Staff_talk';
$wgNamespaceAliases['S'] = NS_STAFF;
xvSetAssocTrues('wgNamespacesWithSubpages', [NS_STAFF, NS_STAFF_TALK]);
xvMergeInto('wgNonincludableNamespaces', [NS_STAFF, NS_STAFF_TALK]);

$wgNamespaceAliases['P'] = NS_PROJECT;

// Lockdown
$xvUseLockdown = true;
$wgSpecialPageLockdown['Export'] = ['user'];

$wgNamespacePermissionLockdown[NS_MEMBER]['read'] = ['user'];
$wgNamespacePermissionLockdown[NS_MEMBER]['edit'] = ['user'];
$wgNamespacePermissionLockdown[NS_MEMBER_TALK] = $wgNamespacePermissionLockdown[NS_MEMBER];

$wgNamespacePermissionLockdown[NS_STAFF]['read'] = ['sysop', 'staff'];
$wgNamespacePermissionLockdown[NS_STAFF]['edit'] = ['sysop', 'staff'];
$wgNamespacePermissionLockdown[NS_STAFF_TALK] = $wgNamespacePermissionLockdown[NS_STAFF];

xvSetAssocTrues('wgNamespacesToBeSearchedDefault', [
	NS_MAIN,
	NS_MEMBER,
	NS_MEMBER_TALK,
	NS_PROJECT,
]);

require_once "$xvConfigDirectory/common/GlobalSettings.php";
