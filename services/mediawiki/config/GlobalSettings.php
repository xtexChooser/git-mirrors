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
	'DiscussionTools',
	'Echo',
	'Gadgets',
	'HeaderTabs',
	'ImageMap',
	'InputBox',
	'Interwiki',
	'Linter',
	'LoginNotify',
	'Loops',
	'MassMessage',
	'Math',
	'MobileFrontend',
	'MultimediaViewer',
	'Nuke',
	'OATHAuth',
	'OAuth',
	'PageImages',
	'ParserFunctions',
	'PdfHandler',
	'Popups',
	'ProtectSite',
	'ReplaceText',
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
