<?php
$wgSitename = "幻光电脑社";
$wgMetaNamespace = "FLCC";
$wgLocalInterwikis[] = 'hgdns';
$xvCentralized = false;
$xvUseSelfHostedImages = true;

// Localisation
$wgLanguageCode = 'zh-hans';
xvMergeInto('wgHiddenPrefs', ['language', 'variant', 'noconvertlink']);
$wgLocaltimezone = 'Asia/Shanghai';
xvMergeInto('wgHiddenPrefs', ['date', 'timecorrection']);
$wgDefaultUserOptions['date'] = 'ISO 8601';

// Copyrights
$wgRightsUrl = "/w/FLCC:著作权";
$wgRightsText = "混合许可证";
$wgRightsIcon = '/images/b/ba/FLCC_Wordmark.svg';

// Logo
$wgLogos = [
	'svg' => '/images/2/2a/FLCC_Logo.svg',
	'icon' => '/images/b/b1/FLCC_Logo_512px.png',
	'wordmark' => [
		'1x' => '/images/b/ba/FLCC_Wordmark.svg',
		'src' => '/images/a/ac/FLCC_Wordmark.png',
		'width' => 124,
		'height' => 32,
	],
];
$wgFavicon = '/images/9/95/FLCC_Logo.ico';

// Search Engine Verifications
$wgXensTweaksMetaSiteVerifications = [
	'360-site' => 'c648dc5a048d60a8eecb07e78a1d0b23',
	'baidu' => 'codeva-BqKf8qjZMW',
];

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
$wgExtraNamespaces[NS_MEMBER] = '社员';
$wgExtraNamespaces[NS_MEMBER_TALK] = '社员讨论';
$wgNamespaceAliases['Member'] = NS_MEMBER;
$wgNamespaceAliases['Member_talk'] = NS_MEMBER_TALK;
$wgContentNamespaces[] = NS_MEMBER;
$wgNamespaceAliases['M'] = NS_MEMBER;
$wgNamespaceAliases['MT'] = NS_MEMBER_TALK;
xvSetAssocTrues('wgNamespacesWithSubpages', [NS_MEMBER, NS_MEMBER_TALK]);
xvMergeInto('wgNonincludableNamespaces', [NS_MEMBER, NS_MEMBER_TALK]);
$wgVisualEditorAvailableNamespaces['社员'] = true;

const NS_STAFF = 3002;
const NS_STAFF_TALK = 3003;
$wgExtraNamespaces[NS_STAFF] = '社干';
$wgExtraNamespaces[NS_STAFF_TALK] = '社干讨论';
$wgNamespaceAliases['Staff'] = NS_STAFF;
$wgNamespaceAliases['Staff_talk'] = NS_STAFF_TALK;
$wgContentNamespaces[] = NS_STAFF;
$wgNamespaceAliases['S'] = NS_STAFF;
xvSetAssocTrues('wgNamespacesWithSubpages', [NS_STAFF, NS_STAFF_TALK]);
xvMergeInto('wgNonincludableNamespaces', [NS_STAFF, NS_STAFF_TALK]);
$wgVisualEditorAvailableNamespaces['社干'] = true;

$wgNamespaceAliases['P'] = NS_PROJECT;
$wgNamespaceAliases['U'] = NS_USER;

// Lockdown
$xvUseLockdown = true;
$wgSpecialPageLockdown['Export'] = ['user'];

$wgNamespacePermissionLockdown[NS_MAIN]['edit'] = ['sysop'];

$wgNamespacePermissionLockdown[NS_MEMBER]['read'] = ['user'];
$wgNamespacePermissionLockdown[NS_MEMBER]['edit'] = ['user'];
$wgNamespacePermissionLockdown[NS_MEMBER_TALK] = $wgNamespacePermissionLockdown[NS_MEMBER];

$wgNamespacePermissionLockdown[NS_STAFF]['read'] = ['sysop', 'staff'];
$wgNamespacePermissionLockdown[NS_STAFF]['edit'] = ['sysop', 'staff'];
$wgNamespacePermissionLockdown[NS_STAFF_TALK] = $wgNamespacePermissionLockdown[NS_STAFF];

xvSetAssocTrues('wgNamespacesToBeSearchedDefault', [
	NS_MAIN,
	NS_MEMBER,
	NS_PROJECT,
]);

// Uploads
xvMergeInto('wgFileExtensions', ['pdf']);

// ContactPage
$xvUseContactPage = true;
$wgContactConfig['join'] = [
	'RecipientUser' => 'Xtex',
	'SenderName' => '',
	'RequireDetails' => false,
	'SubjectReadonly' => true,
	'AdditionalFields' => [
		'EndInfo' => [
			'label' => '本年度招新报名已结束。如有意愿加入社团请直接联系社干。',
			'type' => 'info',
			'raw' => true,
		],
	],
];

$wgContactConfig['interview'] = [
	'RecipientUser' => 'Xtex',
	'SenderName' => '',
	'RequireDetails' => false,
	'SubjectReadonly' => true,
	'AdditionalFields' => [
		'EndInfo' => [
			'label' => '本年度招新报名已结束。如有意愿加入社团请直接联系社干。',
			'type' => 'info',
			'raw' => true,
		],
	],
];

require_once "$xvConfigDirectory/common/GlobalSettings.php";
