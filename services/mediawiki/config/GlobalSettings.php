<?php

$wgScript = '/';
$wgScriptPath = '';
$wgUsePathInfo = true;
$wgArticlePath = '/w/$1';

$wgLanguageCode = 'en';

require_once (dirname(__FILE__) . '/Database.php');

wfLoadSkin('Vector');
$wgDefaultSkin = 'vector-2022';
