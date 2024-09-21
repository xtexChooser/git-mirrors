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
			'label' => '姓名？',
			'type' => 'text',
			'required' => true,
		],
		'departments' => [
			'class' => 'HTMLMultiSelectField',
			'label' => '希望加入的部门',
			'options' => [
				'编程部' => 'depart-sw',
				'硬件部' => 'depart-hw',
				'有意愿成为社干' => 'want-leader',
			]
		],
		'contact' => [
			'label' => '联系方式（微信/QQ均可）',
			'type' => 'text',
			'required' => true,
		],
	],
	'DisplayFormat' => 'table',
	'RLModules' => [],
	'RLStyleModules' => [],
];

$wgContactConfig['interview'] = [
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
			'help' => '记得写学号，好多人报名表只写了班级。',
			'type' => 'text',
			'required' => true,
		],
		'departments' => [
			'type' => 'multiselect',
			'label' => '希望加入的部门',
			'options' => [
				'编程部' => 'depart-sw',
				'硬件部' => 'depart-hw',
				'有意愿成为社干' => 'want-leader',
			],
			'required' => false,
		],
		'experience' => [
			'type' => 'multiselect',
			'label' => '请选择正确的：',
			'options' => [
				'有兴趣学习编程' => 'want-learn',
				'我觉得我还算比较会用电脑' => 'computer-ok',
				'大致了解计算机各组件的作用' => 'know-usage',
				'以前学习过编程（C/C++）' => 'previous-learnt-cpp',
				'以前学习过编程（其他语言）' => 'previous-learnt-other',
				'以前参加过相关比赛（CSP-J/S或NOI系列）' => 'previous-competition-noi',
				'以前参加过相关比赛（其他）' => 'previous-competition-other',
				'真的真的很想学编程或装机' => 'really-want',
				'真的真的真的很想学编程' => 'really-really-want',
				'参加过一中的计算机或数学物理自主招生考试' => 'yz-contest',
				'我就是想来玩游戏的' => 'just-gaming',
				'1+1=2' => 'math1',
				'(16 >>> 3) = 1' => 'math2',
				'PHP是世界上最好的语言' => 'php-is-best',
				'阳江一中是世界上最好的学校' => 'yjyz-is-best',
				'你说的对' => 'you-r-right',
				'你说的不对' => 'you-r-not-right',
				'学校的希沃是2019年的' => '2019-seewo',
			],
			'required' => false,
		],
		'c1c2' => [
			'type' => 'multiselect',
			'label-message' => 'contactpage-interview-math',
			'help' => '说实话出题人也不会这个，也不指望有人能做出来，这是数学选必二的内容，所以不会就直接留空吧',
			'options' => [
				'2' => 'c1c2-a',
				'4' => 'c1c2-b',
			],
			'required' => false,
		],
		'time-complexity' => [
			'type' => 'multiselect',
			'label' => '假设一个长度为n的整数数组中每个元素值互不相同，且这个数组是无序的。要找到这个数组中的最大元素的时间复杂度是多少？',
			'help' => '也不指望有人能做出来，这是今年CSP-J/S竞赛S组初赛的题目，所以不会就直接留空吧，学一学期就会了',
			'options' => [
				'O(n)' => 'time-comp-n',
				'O(log n)' => 'time-comp-log-n',
				'O(n log n)' => 'time-comp-n-log-n',
				'O(1)' => 'time-comp-unit',
			],
			'required' => false,
		],
		'other' => [
			'label' => '还有什么想说的吗',
			'help' => '这个也可以留空的',
			'type' => 'textarea',
			'rows' => 7,
			'required' => false,
		],
		'luogu' => [
			'label' => '洛谷UID（如果有的话）',
			'help' => '这个我也做好了全是空的的准备',
			'type' => 'text',
			'required' => false,
		],
		'wiki-username' => [
			'label' => '选个用户名吧',
			'help' => '面试通过后会在本网站创建一个账号（虽然没什么用），中文英文数字空格都可以有',
			'type' => 'text',
			'required' => true,
		],
		'polface' => [
			'label' => '政治面貌',
			'help' => '是团员写团员，不是团员写群众，学校要统一收集的，不影响面试',
			'type' => 'text',
			'required' => true,
		],
		'contact' => [
			'label' => '个人联系方式',
			'help' => '这个也是学校要统一收集的，留个电话也行',
			'type' => 'text',
			'required' => true,
		],
	],
	'DisplayFormat' => 'table',
	'RLModules' => [],
	'RLStyleModules' => [],
];

require_once "$xvConfigDirectory/common/GlobalSettings.php";
