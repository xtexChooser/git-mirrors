<?php

$wgScript = '/';
$wgScriptPath = '';
$wgUsePathInfo = true;
$wgArticlePath = '/w/$1';

$wgLanguageCode = 'en';
$wgLocaltimezone = 'UTC';

require_once (dirname(__FILE__) . '/Database.php');

$xvLoadSkins[] = 'Vector';
$wgDefaultSkin = 'vector-2022';

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
	'ConfirmEdit',
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
