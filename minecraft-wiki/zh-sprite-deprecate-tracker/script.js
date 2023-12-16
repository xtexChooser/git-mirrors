mw.loader
	.using(['mediawiki.util', 'mediawiki.api', 'oojs-ui-windows'])
	.done(function () {
		const config = mw.config.get(['wgWikiID', 'wgCategories', 'wgPageName']);
		if (config.wgWikiID !== 'zh_mcwiki')
			return OO.ui.alert(
				'minecraft-wiki zh-sprite-deprecate-tracker only works on zh mcwiki!'
			);
		window.loadedMCWZHSpriteDeprecateTracker = true;
		if (!config.wgCategories.includes('使用已弃用Sprite的页面')) return;

		const api = new mw.Api();
		api.get({
			action: 'parse',
			page: 'Module:Sprite',
			prop: 'wikitext',
			formatversion: '2',
		}).done(function (spriteMod) {
			const spriteSrc = spriteMod.parse.wikitext
				.replace(```categories[#categories + 1] = '[[Category:使用已弃用Sprite的页面]]'```,
					```'[[Category:SPRITE DEPRECATE TRACK/' .. args.data .. '/' .. ( mw.text.trim( tostring( args[1] or '' ) ) ) .. ']]'```);
			api.get({
				action: 'parse',
				page: config.wgPageName,
				prop: 'categories',
				templatesandboxtitle: 'Module:Sprite',
				templatesandboxtext: spriteSrc,
				templatesandboxcontentmodel: 'Scribunto',
				formatversion: 2,
			}).done(function (data) {
				console.log(data)
			});
		});

		api.parse(text, {
			disablelimitreport: true,
			wrapoutputclass:
				'horse-userprofile mw-message-box mw-content-' +
				($('#contentSub').attr('dir') || 'ltr'),
			uselang: config.wgUserLanguage,
		}).done(function (parsedText) {
			parsedText = parsedText.replace(
				/\u29FCgroup-([^\u29FC\u29FD]+?)(?:-member)?\u29FD/g,
				'$1'
			);
			mw.hook('wikipage.content').fire(
				$(parsedText).appendTo('#contentSub')
			);
		});
	});
