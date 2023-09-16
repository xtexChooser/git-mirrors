<?php
if ( !defined( 'MEDIAWIKI' ) ) {
	exit;
}

$wgSitename = "XV-NET Wiki";
$wgMetaNamespace = "XV-NET_Wiki";

$wgScriptPath = "";
$wgArticlePath = "/wiki/$1";
$wgUsePathInfo = true;

## The protocol and server name to use in fully-qualified URLs
$wgServer = "https://w.xvnet.eu.org";

$wgResourceBasePath = $wgScriptPath;

$wgLogos = [
	'1x' => "$wgResourceBasePath/resources/assets/xvnet-logo.svg",
	'icon' => "$wgResourceBasePath/resources/assets/xvnet-logo.svg",
];

$wgEnableEmail = true;
$wgEnableUserEmail = true;

$wgEmergencyContact = "emerg@xvnet.eu.org";
$wgPasswordSender = "noreply@xvnet.eu.org";

$wgEnotifUserTalk = true;
$wgEnotifWatchlist = true;
$wgEmailAuthentication = true;

## Database settings
$wgDBtype = "postgres";
$wgDBserver = "pgsql.xvnet.eu.org";
$wgDBname = "xvnetwiki";
$wgDBuser = "mediawiki";
$wgDBpassword = "{{ pillar['mediawiki']['db_pwd'] }}";
$wgDBport = "5432";
$wgDBmwschema = "mediawiki";

# Shared database table
# This has no effect unless $wgSharedDB is also set.
$wgSharedTables[] = "actor";

## Shared memory settings
$wgMainCacheType = CACHE_ACCEL;
$wgMemCachedServers = [];

## To enable image uploads, make sure the 'images' directory
## is writable, then set this to true:
$wgEnableUploads = true;
$wgUseImageMagick = true;
$wgImageMagickConvertCommand = "/usr/bin/convert";

# InstantCommons allows wiki to use images from https://commons.wikimedia.org
$wgUseInstantCommons = true;

# Periodically send a pingback to https://www.mediawiki.org/ with basic data
# about this MediaWiki instance. The Wikimedia Foundation shares this data
# with MediaWiki developers to help guide future development efforts.
$wgPingback = true;

# Site language code, should be one of the list in ./includes/languages/data/Names.php
$wgLanguageCode = "en";

# Time zone
$wgLocaltimezone = "UTC";

## Set $wgCacheDirectory to a writable directory on the web server
## to make your wiki go slightly faster. The directory should not
## be publicly accessible from the web.
#$wgCacheDirectory = "$IP/cache";

$wgSecretKey = "{{ pillar['mediawiki']['secret_key'] }}";
$wgAuthenticationTokenVersion = "1";
$wgUpgradeKey = false;

#$wgRightsPage = "";
$wgRightsUrl = "https://creativecommons.org/licenses/by-sa/4.0/";
$wgRightsText = "知识共享署名-相同方式共享 4.0 国际";
$wgRightsIcon = "$wgResourceBasePath/resources/assets/licenses/cc-by-sa.png";

$wgDiff3 = "/usr/bin/diff3";

$wgGroupPermissions['*']['edit'] = false;

$wgDefaultSkin = "vector";

wfLoadSkin( 'MinervaNeue' );
wfLoadSkin( 'Timeless' );
wfLoadSkin( 'Vector' );

wfLoadExtension( 'AbuseFilter' );
wfLoadExtension( 'CategoryTree' );
wfLoadExtension( 'Cite' );
wfLoadExtension( 'CiteThisPage' );
wfLoadExtension( 'CodeEditor' );
wfLoadExtension( 'Gadgets' );
wfLoadExtension( 'InputBox' );
wfLoadExtension( 'Interwiki' );
wfLoadExtension( 'Math' );
wfLoadExtension( 'MultimediaViewer' );
wfLoadExtension( 'Nuke' );
wfLoadExtension( 'OATHAuth' );
wfLoadExtension( 'PageImages' );
wfLoadExtension( 'ParserFunctions' );
wfLoadExtension( 'PdfHandler' );
wfLoadExtension( 'Renameuser' );
wfLoadExtension( 'ReplaceText' );
wfLoadExtension( 'Scribunto' );
wfLoadExtension( 'SecureLinkFixer' );
wfLoadExtension( 'SyntaxHighlight_GeSHi' );
wfLoadExtension( 'TemplateData' );
wfLoadExtension( 'TextExtracts' );
wfLoadExtension( 'VisualEditor' );
wfLoadExtension( 'WikiEditor' );
