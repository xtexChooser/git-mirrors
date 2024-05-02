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
			'.mcwzh-sprite-deprecate-tracker { background: none; padding: 0.5em; }\n\
			.mcwzh-sprite-deprecate-data { display: none; }\n\
			.mcwzh-sprite-deprecate-alter { padding: 1em; }'
		);

		const api = new mw.Api();

		function normKey(str) {
			return str.toLowerCase().replaceAll(/[\s_+]/g, '-');
		}

		function findAlternativeIn(data, name, callback) {
			const ids = data.ids;
			const d = ids[name] || ids[normKey(name)];
			let alts = {};
			for (k in ids) {
				if (k == name) continue;
				if (ids[k].pos == d.pos && !(ids[k].deprecated || false))
					alts[k] = ids[k];
			}
			callback(data, alts);
		}

		let cachedData = {};
		function findAlternative(data, name, callback) {
			if (data in cachedData) {
				findAlternativeIn(cachedData[data], name, callback);
			} else {
				api.postWithToken('csrf', {
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

		function showAlternative(data, alts) {
			let text = '';
			if (Object.keys(alts).length == 0) text = '<无>';
			for (k in alts) {
				text += k;
				if ('section' in alts[k]) {
					let section = '<错误>';
					for (sec of data.sections) {
						if (sec.id == alts[k].section) {
							section = sec.name;
							break;
						}
					}
					text += `（${section}）`;
				}
				text += '\n';
			}
			OO.ui.alert(text, { size: 'large', title: '替代' });
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
				}).done(function (respdata) {
					const parsed = respdata.parse.categories
						.map((c) => c.category)
						.filter((c) => c.startsWith(catPrefix))
						.map((c) => c.substring(catPrefix.length + 1));
					let data = {};
					for (const c of parsed) {
						const dataset = c.substring(0, c.indexOf('/'));
						const name = c.substring(c.indexOf('/') + 1);
						if (!(dataset in data)) {
							data[dataset] = [];
						}
						data[dataset].push(name);
					}
					let text = '';
					for (const c in data) {
						text += `* [[Module:${c}|${c}]]\n`;
						for (const n of data[c]) {
							text += `** <code>${n}</code><span class="mcwzh-sprite-deprecate-data">${c}/${n}</span><span class="mcwzh-sprite-deprecate-alter"></span>\n`;
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
					$('.mcwzh-sprite-deprecate-alter').each(
						function (_i, alter) {
							const text =
								alter.parentNode.childNodes[1].innerText;
							const data = text.substring(0, text.indexOf('/'));
							const name = text.substring(text.indexOf('/') + 1);

							const button = new OO.ui.ButtonWidget({
								framed: false,
								label: '替代',
							});
							button.on('click', function () {
								findAlternative(data, name, showAlternative);
							});
							$(button.$element).appendTo(alter);
						}
					);
				});
			});
		});
	});
