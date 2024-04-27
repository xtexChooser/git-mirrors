## GloopTweaks
This extension handles site-wide interface messages, tweaks, and improvements on Weird Gloop wikis. It is a fork of Wikimedia's WikimediaMessages with increased functionality, handling server-farm wide modifications to the core MediaWiki software.

### Rights
* `editinterfacesite` - Allows the user to edit site-wide interface messages if `$wgGloopTweaksProtectSiteInterface` is enabled

### Configuration settings
* `$wgGloopTweaksProtectSiteInterface` - Protect site-wide interface messages from local edits
* `$wgGloopTweaksSendDetailsWithContactPage` - Send additional info with Special:Contact messages (e.g IP and User-Agent)
* `$wgGloopTweaksAddFooterLinks` - Add a link to our Terms of Use etc to the footer
* `$wgGloopTweaksEnableMessageOverrides` - Enable overriding certain MediaWiki messages with our own
* `$wgGloopTweaksRequireLicensesToUpload` - Enforces requiring MediaWiki:Licenses to not be blank for uploads to be enabled
* `$wgGloopTweaksEnableSearchboxMetadata` - Add structured data to the main page to add a Google Sitelinks search box
* `$wgGloopTweaksEnableTheming` - Enable loading themes when the theme cookie is set or the legacy darkmode cookie is true
* `$wgGloopTweaksEnableLoadingFixedWidth` - Enable loading the wg.fixedwidth ResourceLoader module when the readermode cookie is true
