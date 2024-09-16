<?php
use MediaWiki\MediaWikiServices;

if ($xvUseDatabaseMonto)
	xvLoadConfig('common/Database.php');

// global extensions
if ($xvUseGlobalExtensions)
	wfLoadExtensions($xvGlobalExtensions);

// global skins
if ($xvUseGlobalSkins)
	wfLoadSkins($xvGlobalSkins);

// emergeny locks
if ($xvEmergAbuse) {
	xvMergeInto2('wgRateLimits', 'purge', [
		'ip' => [10, 60],
		'newbie' => [10, 45],
	]);
	xvMergeInto2('wgRateLimits', 'linkpurge', [
		'ip' => [10, 60],
		'newbie' => [10, 45],
	]);
	xvMergeInto2('wgRateLimits', 'renderfile-nonstandard', [
		'ip' => [70, 300],
		'newbie' => [100, 300],
	]);
	xvMergeInto2('wgRateLimits', 'badcaptcha', [
		'ip' => [10, 3600],
		'newbie' => [15, 3600],
	]);
	xvMergeInto2('wgRateLimits', 'sendemail', [
		'ip' => [2, 86400],
		'newbie' => [2, 86400],
	]);
}
if ($xvEmergSecLockdown) {
	$wgMWOAuthReadOnly = true;
}

// Centralization
if ($xvCentralized)
	xvLoadConfig('common/Centralization.php');

// CAPTCHA
if ($xvUseCaptcha) {
	wfLoadExtension('ConfirmEdit');

	switch ($xvCaptchaType) {
		case XvCaptchaType::Turnstile:
			wfLoadExtension('ConfirmEdit/Turnstile');
			$wgTurnstileSendRemoteIP = false;
			break;
		case XvCaptchaType::Questy:
			wfLoadExtension('ConfirmEdit/QuestyCaptcha');
			break;
	}

	xvSetAssocTrues('wgCaptchaTriggers', [
		'create',
		'sendemail',
		'addurl',
		'createaccount',
		'badlogin',
		'badloginperuser'
	]);
	if ($xvCaptchaOnEdit || $xvEmergAbuse)
		$wgCaptchaTriggers['edit'] = true;

	xvGrantPermission('skipcaptcha', ['staff', 'sysop', 'bureaucrat', 'bot']);
	if ($xvSkipCaptchaForAutoconfirmed) {
		$wgGroupPermissions['autoconfirmed']['skipcaptcha'] = true;
		if ($wgAllowConfirmedEmail) {
			$wgGroupPermissions['emailconfirmed']['skipcaptcha'] = true;
		}
	}
}

if (xvIsExtensionLoaded('BetaFeatures')) {
	$wgConditionalUserOptions['betafeatures-auto-enroll'] = [
		[1, [CUDCOND_USERGROUP, 'sysop']],
		[1, [CUDCOND_USERGROUP, 'staff']],
	];
}

if ($xvUseVisualEditor) {
	wfLoadExtension('VisualEditor');
	$wgVisualEditorEnableWikitext = true;
	$wgVisualEditorEnableTocWidget = true;

	xvSetAssocTrues('wgVisualEditorAvailableNamespaces', $xvVisualEditorNamespaces);

	if ($xvUseVisualEditorAsDefault) {
		$wgVisualEditorUseSingleEditTab = true;
		$wgDefaultUserOptions['visualeditor-editor'] = 'visualeditor';
		$wgDefaultUserOptions['visualeditor-newwikitext'] = 1;
	}
}

if (xvIsExtensionLoaded('CodeMirror')) {
	$wgDefaultUserOptions['usecodemirror'] = true;
}

if (xvIsExtensionLoaded('TemplateSandbox')) {
	$wgTemplateSandboxEditNamespaces = [NS_TEMPLATE];
	if (xvIsExtensionLoaded('Scribunto'))
		$wgTemplateSandboxEditNamespaces[] = NS_MODULE;
}

if ($xvUseMobileFrontend) {
	wfLoadExtension('MobileFrontend');
	$wgDefaultMobileSkin = $xvDefaultMobileSkin;
	$wgMFSiteStylesRenderBlocking = true;
}

if (xvIsExtensionLoaded('ThrottleOverride')) {
	$wgGroupPermissions['staff']['throttleoverride'] = true;

	xvSetAssocTrues('wgThrottleOverrideTypes', [
		'actcreate',
		'edit',
		'move',
		'emailuser'
	]);
}

if (xvIsExtensionLoaded('OAuth')) {
	xvMergeInto('wgOAuthGroupsToNotify', ['sysop', 'staff']);

	xvGrantPermission('mwoauthproposeconsumer', ['autoconfirmed', 'sysop', 'staff']);
	xvGrantPermission('mwoauthmanageconsumer', ['sysop', 'bureaucrat', 'staff']);
	xvGrantPermission('mwoauthsuppress', ['sysop', 'bureaucrat', 'staff']);
	xvGrantPermission('mwoauthviewsuppressed', ['staff']);
	xvGrantPermission('mwoauthviewprivate', ['staff']);

	$wgOAuth2PublicKey = <<<EOF
-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE6dk4RIVV3eVNbHBd/8ov7i/ljQFu
U/a6RwrI5yo5PRbTGYAaz5cdGnNjbkq3CCEc0Fd6LH9w0LU0d1uSDOS5vw==
-----END PUBLIC KEY-----
EOF;
}

if ($wgUseSharedUploads) {
	if ($xvUseWikimediaCommons) {
		$wgForeignFileRepos[] = [
			'class' => ForeignAPIRepo::class,
			'name' => 'wikimediacommons',
			'apibase' => 'https://commons.wikimedia.org/w/api.php',
			'url' => 'https://upload.wikimedia.org/wikipedia/commons',
			'thumbUrl' => 'https://upload.wikimedia.org/wikipedia/commons/thumb',
			'hashLevels' => 2,
			'transformVia404' => true,
			'fetchDescription' => true,
			'descriptionCacheExpiry' => 43200,
			'apiMetadataExpiry' => 28800,
			'apiThumbCacheExpiry' => 0,
		];
	}
	if ($xvUseMirahezeCommons) {
		$wgForeignFileRepos[] = [
			'class' => ForeignAPIRepo::class,
			'name' => 'mirahezecommons',
			'apibase' => 'https://commons.miraheze.org/w/api.php',
			'url' => 'https://static.miraheze.org/commonswiki',
			'thumbUrl' => 'https://static.miraheze.org/commonswiki/thumb',
			'hashLevels' => 2,
			'transformVia404' => true,
			'fetchDescription' => true,
			'descriptionCacheExpiry' => 43200,
			'apiMetadataExpiry' => 28800,
			'apiThumbCacheExpiry' => 0,
		];
	}
}

if (xvIsExtensionLoaded('Echo')) {
	$wgNotificationSenderName = $xvNotificationSenderName ?? $wgSitename;
}

if ($xvEmergSecLockdown) {
	$allPermissions = MediaWikiServices::getInstance()->getPermissionManager()->getAllPermissions();
	xvRemovePermissionsFrom('*', $allPermissions);
	xvGrantPermissionsTo('staff', $allPermissions);
}
