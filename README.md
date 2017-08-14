# CategoryWatch

Main development Git repository on Gerrit: https://gerrit.wikimedia.org/r/#/admin/projects/mediawiki/extensions/CategoryWatch

Here is the Phabricator Diffusion: https://phabricator.wikimedia.org/diffusion/ECWA/

Mirror on GitHub: https://github.com/seanchen/CategoryWatch

MediaWiki extension CategoryWatch, https://www.mediawiki.org/wiki/Extension:CategoryWatch

Initial commit is forked from commit 4ad0f63:
https://github.com/OrganicDesign/extensions/tree/4ad0f631438ed16c05edfa08e65e0de00b4b1342/MediaWiki-Legacy/CategoryWatch

## Configurables

Whether or not to also send notificaton to the person who made the change.
```php
$wgCategoryWatchNotifyEditor = true;
```

Give every user a unique category that they're automatically watching. The format of the category name is defined on the "categorywatch-autocat" localisation message (i.e. [[MediaWiki:categorywatch-autocat]])
```php
$wgCategoryWatchUseAutoCat = true;
```

Make the categorisation work by realname instead of username
```php
$wgCategoryWatchUseAutoCatRealName = true;
```

Notify watchers of parent categories as well
```php
$wgCategoryWatchNotifyParentWatchers = true;
```

## How to debug

3 simple steps to debug an extension.

* using function wfDebugLog to log message,
  using the extension name as the group. for example:
```php
wfDebugLog('CategoryWatch', 'loading extension...');
```
* enable debug log for the group, bascially the extension name.
  set the log file name.
```php
# in file LocalSettings.php
$wgDebugLogGroups['CategoryWatch'] = '/path/to/log/mw-categorywatch.log';
```
* tail the log file to debug...
