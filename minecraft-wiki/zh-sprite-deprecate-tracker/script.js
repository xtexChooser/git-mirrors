mw.loader
	.using(['mediawiki.util', 'mediawiki.api', 'oojs-ui-windows'])
	.done(function () {
		const config = mw.config.get([
			'wgWikiID',
			'wgCategories',
			'wgPageName',
			'wgUserLanguage',
		]);
		if (config.wgWikiID !== 'zh_mcwiki')
			return OO.ui.alert(
				'minecraft-wiki zh-sprite-deprecate-tracker only works on zh mcwiki!'
			);
		window.loadedMCWZHSpriteDeprecateTracker = true;
		if (!config.wgCategories.includes('使用已弃用Sprite的页面')) return;

		mw.util.addCSS(
			'.mcwzh-sprite-deprecate-tracker { background: none; padding: 0.5em; }'
		);

		const api = new mw.Api();
		api.get({
			action: 'parse',
			page: 'Module:Sprite',
			prop: 'wikitext',
			formatversion: '2',
		}).done(function (spriteMod) {
			const spriteSrc = spriteMod.parse.wikitext.replace(
				"categories[#categories + 1] = '[[Category:使用已弃用Sprite的页面]]'",
				"categories[#categories + 1] = '[[Category:SPRITE DEPRECATE TRACK/' .. args.data .. '/' .. ( mw.text.trim( tostring( args[1] or '' ) ) ) .. ']]'"
			);
			api.post({
				action: 'parse',
				page: config.wgPageName,
				prop: 'categories',
				templatesandboxtitle: 'Module:Sprite',
				templatesandboxtext: spriteSrc,
				templatesandboxcontentmodel: 'Scribunto',
				formatversion: 2,
			}).done(function (data) {
				const catPrefix = 'SPRITE DEPRECATE TRACK';
				const parsed = data.parse.categories
					.map((c) => c.category)
					.filter((c) => c.startsWith(catPrefix))
					.map((c) => c.substring(catPrefix.length + 1));
				var deprecated = {};
				var text = '已弃用精灵图：\n';
				for (c of parsed) {
					const dataset = c.substring(0, c.indexOf('/'));
					const name = c.substring(c.indexOf('/') + 1);
					if (!(dataset in deprecated)) {
						deprecated[dataset] = [];
					}
					deprecated[dataset].push(name);
				}
				for (c in deprecated) {
					text += `* ${c}\n`;
					for (n of deprecated[c]) {
						text += `** ${n}\n`;
					}
				}

				api.parse(text, {
					disablelimitreport: true,
					wrapoutputclass:
						'mcwzh-sprite-deprecate-tracker mw-message-box mw-content-' +
						($('#contentSub').attr('dir') || 'ltr'),
					uselang: config.wgUserLanguage,
				}).done(function (pt) {
					mw.hook('wikipage.content').fire(
						$(pt).appendTo('#bodyContent')
					);
				});
			});
		});
	});
