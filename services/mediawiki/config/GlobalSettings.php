<?php

$wgServerName = $xvWikis[$xvWikiID];
$wgServer = 'https://' . $wgServerName;

if ($xvMaintScript) {
	// Disable [[MW:*]] messages in maintenance script
	$wgUseDatabaseMessages = false;
}

$wgScript = '/';
$wgScriptPath = '';
$wgUsePathInfo = true;
$wgArticlePath = '/w/$1';

// Default Localisation
$wgLanguageCode = 'en';
$wgLocaltimezone = 'UTC';

require_once (dirname(__FILE__) . '/Database.php');

// Global Extensions
$xvLoadExtensions = array_merge($xvLoadExtensions, [
	'AbuseFilter',
	'AntiSpoof',
	'CategoryTree',
	'CheckUser',
	'Cite',
	'CiteThisPage',
	'cldr',
	'CodeEditor',
	'CodeMirror',
	'Description2',
	'DiscussionTools',
	'DynamicPageList3',
	'Echo',
	'Gadgets',
	'HeaderTabs',
	'ImageMap',
	'InputBox',
	'Interwiki',
	'JsonConfig',
	'Linter',
	'LoginNotify',
	'Loops',
	'MassMessage',
	'Math',
	'MobileFrontend',
	'MultimediaViewer',
	'NoTitle',
	'Nuke',
	'OATHAuth',
	'OAuth',
	'PageImages',
	'ParserFunctions',
	'PdfHandler',
	'Popups',
	'ProtectSite',
	'RevisionSlider',
	'Scribunto',
	'SecureLinkFixer',
	'SpamBlacklist',
	'SyntaxHighlight_GeSHi',
	'TemplateData',
	'TemplateStyles',
	'TextExtracts',
	'TitleBlacklist',
	'VisualEditor',
	'WikiEditor',
]);

// Default Skin
$xvLoadSkins[] = 'Vector';
$wgDefaultSkin = 'vector-2022';

// Rate Limits
$wgRateLimits['purge']['user'] = [30, 30];
$wgRateLimits['linkpurge']['user'] = [30, 30];
$wgRateLimits['renderfile-nonstandard']['user'] = [100, 30];
$wgRateLimits['badcaptcha']['newbie'] = [50, 86400];

// Confirm Edit
$xvLoadExtensions[] = 'ConfirmEdit';
$wgGroupPermissions['autoconfirmed']['skipcaptcha'] = true;
$wgGroupPermissions['emailconfirmed']['skipcaptcha'] = true;
$wgCaptchaTriggers['create'] = true;
$wgCaptchaTriggers['sendemail'] = true;

$xvLoadExtensions[] = 'ConfirmEdit/Turnstile';
$wgTurnstileSendRemoteIP = false;

// BetaFeatures
$xvLoadExtensions[] = 'BetaFeatures';
// TODO: enable betafeatures by default for sysops
// https://gerrit.wikimedia.org/r/c/mediawiki/core/+/1022496

// Misc
$wgNamespacesWithSubpages[NS_MAIN] = true;
$wgEnableMetaDescriptionFunctions = true;
$wgGroupPermissions['staff']['interwiki'] = true;
$wgDefaultUserOptions['usecodemirror'] = true;
$wgJsonConfigEnableLuaSupport = true;
$wgAllowCrossOrigin = true;
$wgAllowUserCss = true;
$wgAllowUserJs = true;
ini_set('user_agent', 'Xens Wikis (op@xvnet0.eu.org');
$wgMainCacheType = CACHE_ACCEL;

// Shared Uploads
$wgUseSharedUploads = true;
$wgForeignFileRepos[] = [
	'class' => ForeignAPIRepo::class,
	'name' => 'commonswiki',
	'apibase' => 'https://commons.wikimedia.org/w/api.php',
	'hashLevels' => 2,
	'fetchDescription' => true,
	'descriptionCacheExpiry' => 43200,
	'apiMetadataExpiry' => 28800,
	'apiThumbCacheExpiry' => 86400,
];

// Email
$wgEnableEmail = true;
$wgSMTP = [
	'host' => 'smtp-mail.outlook.com',
	'IDHost' => $wgServerName,
	'localhost' => $wgServerName,
	'port' => 587,
	'auth' => true,
	'username' => $xvSMTPUsername,
	'password' => $xvSMTPPassword
];
$wgEmergencyContact = $xvOpContactEmail;
$wgPasswordSender = $xvSMTPUsername;

// Lua
$wgScribuntoDefaultEngine = 'luasandbox';
$wgScribuntoEngineConf['luastandalone']['luaPath'] = '/usr/bin/lua';
$wgScribuntoEngineConf['luasandbox']['memoryLimit'] = 50 * 1024 * 1024;
$wgScribuntoEngineConf['luasandbox']['cpuLimit'] = 10;

// Uploads
$wgEnableUploads = true;
$wgAllowCopyUploads = true;
$wgCopyUploadsFromSpecialUpload = true;
$wgGroupPermissions['autoconfirmed']['upload_by_url'] = true;
