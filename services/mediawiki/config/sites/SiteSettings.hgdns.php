<?php
$wgSitename = "幻光电脑社";
$wgMetaNamespace = "Project";
$wgLocalInterwikis[] = 'hgdns';
$xvCentralized = false;

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
$wgExtraNamespaces[NS_MEMBER] = '社员';
$wgExtraNamespaces[NS_MEMBER_TALK] = '社员讨论';
$wgNamespaceAliases['Member'] = NS_MEMBER;
$wgNamespaceAliases['Member_talk'] = NS_MEMBER_TALK;
$wgContentNamespaces[] = NS_MEMBER;
$wgNamespaceAliases['M'] = NS_MEMBER;
$wgNamespaceAliases['MT'] = NS_MEMBER_TALK;
xvSetAssocTrues('wgNamespacesWithSubpages', [NS_MEMBER, NS_MEMBER_TALK]);
xvMergeInto('wgNonincludableNamespaces', [NS_MEMBER, NS_MEMBER_TALK]);

const NS_STAFF = 3002;
const NS_STAFF_TALK = 3003;
$wgExtraNamespaces[NS_STAFF] = '社干';
$wgExtraNamespaces[NS_STAFF_TALK] = '社干讨论';
$wgNamespaceAliases['Staff'] = NS_STAFF;
$wgNamespaceAliases['Staff_talk'] = NS_STAFF_TALK;
$wgNamespaceAliases['S'] = NS_STAFF;
xvSetAssocTrues('wgNamespacesWithSubpages', [NS_STAFF, NS_STAFF_TALK]);
xvMergeInto('wgNonincludableNamespaces', [NS_STAFF, NS_STAFF_TALK]);

$wgNamespaceAliases['P'] = NS_PROJECT;

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

// ContactPage
$xvUseContactPage = true;
$wgContactConfig['join'] = [
	'RecipientUser' => 'Xtex',
	'SenderName' => '',
	'RequireDetails' => true,
	'IncludeIP' => false,
	'MustBeLoggedIn' => false,
	'NameReadonly' => false,
	'EmailReadonly' => false,
	'SubjectReadonly' => true,
	'MustHaveEmail' => false,
	'AdditionalFields' => [
		'stunum' => [
			'label' => '班级与学号？',
			'type' => 'text',
			'required' => true,
		],
		'name' => [
			'label' => '姓名',
			'type' => 'text',
			'required' => true,
		],
		'departments' => [
			'class' => 'HTMLMultiSelectField',
			'label' => '加入的部门',
			'options' => [
				'编程部' => 'depart-sw',
				'硬件部' => 'depart-hw',
				'有意愿成为社干' => 'want-leader',
			]
		],
		'contact' => [
			'label' => '联系方式',
			'type' => 'text',
			'required' => true,
		],
	],
	'DisplayFormat' => 'table',
	'RLModules' => [],
	'RLStyleModules' => [],
];

require_once "$xvConfigDirectory/common/GlobalSettings.php";
