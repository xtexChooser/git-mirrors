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

		function findAlternativeIn(data, name, callback) {}

		var cachedData = {};
		function findAlternative(data, name, callback) {
			if (data in cachedData) {
				findAlternativeIn(cachedData[data], name, callback);
			} else {
				api.get({
					action: 'scribunto-console',
					title: 'Module:Sprite',
					question: `print( mw.text.jsonEncode( mw.loadData( 'Module:${data}' ) ) )`,
					formatversion: '2',
				}).done(function (resp) {
					const out = JSON.parse(resp.print);
					cachedData[data] = out;
					findAlternativeIn(out, name, callback);
				});
			}
		}

		api.get({
			action: 'parse',
			page: 'Module:Sprite',
			prop: 'wikitext',
			formatversion: '2',
		}).done(function (spriteMod) {
			const spriteSrc = spriteMod.parse.wikitext.replace(
				"categories[#categories + 1] = '[[Category:使用已弃用Sprite的页面]]'",
				"categories[#categories + 1] = '[[Category:SPRITE_DEPRECATE_TRACK/' .. args.data .. '/' .. ( mw.text.trim( tostring( args[1] or '' ) ) ) .. ']]'"
			);
			const catPrefix = 'SPRITE_DEPRECATE_TRACK';
			function generateReport(page, callback) {
				api.post({
					action: 'parse',
					page: page,
					prop: 'categories',
					templatesandboxtitle: 'Module:Sprite',
					templatesandboxtext: spriteSrc,
					templatesandboxcontentmodel: 'Scribunto',
					formatversion: 2,
				}).done(function (data) {
					const parsed = data.parse.categories
						.map((c) => c.category)
						.filter((c) => c.startsWith(catPrefix))
						.map((c) => c.substring(catPrefix.length + 1));
					var data = {};
					for (const c of parsed) {
						const dataset = c.substring(0, c.indexOf('/'));
						const name = c.substring(c.indexOf('/') + 1);
						if (!(dataset in data)) {
							data[dataset] = [];
						}
						data[dataset].push(name);
					}
					var text = '';
					for (const c in data) {
						text += `* [[Module:${c}|${c}]]\n`;
						for (const n of data[c]) {
							text += `** <code>${n}</code><span class="mcwzh-sprite-deprecate-alter" x-data-mod="${c}" x-name="${n}"></span>\n`;
						}
					}

					callback(text);
				});
			}
			generateReport(config.wgPageName, function (text) {
				api.parse('已弃用精灵图：\n' + text, {
					disablelimitreport: true,
					wrapoutputclass:
						'mcwzh-sprite-deprecate-tracker mw-message-box mw-content-' +
						($('#contentSub').attr('dir') || 'ltr'),
					uselang: config.wgUserLanguage,
				}).done(function (pt) {
					mw.hook('wikipage.content').fire(
						$(pt).appendTo('#bodyContent')
					);
					$('.mcwzh-sprite-deprecate-alter').each(function (alter) {
						const button = new OO.ui.ButtonWidget({
							framed: false,
							label: '替代',
						});
						const el = alter;
						button.on('click', function () {
							console.log(el);
						});
						$(alter).append(button.$element);
					});
				});
			});
		});
	});
