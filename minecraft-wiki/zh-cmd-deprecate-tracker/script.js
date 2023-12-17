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
				'minecraft-wiki zh-cmd-deprecate-tracker only works on zh mcwiki!'
			);
		window.loadedMCWZHCmdDeprecateTracker = true;
		if (!config.wgCategories.includes('使用了Command模块已弃用功能的页面'))
			return;

		mw.util.addCSS(
			'.mcwzh-cmd-deprecate-tracker { background: none; padding: 0.5em; }\n\
			.mcwzh-cmd-deprecate-data { display: none; }'
		);

		const REASONS = {
			FULLCMD:
				'<code>?</code>和<code>...</code>已被[[Special:Diff/678702|弃用]]，暂无替代',
		};

		const api = new mw.Api();

		api.get({
			action: 'parse',
			page: 'Module:Command',
			prop: 'wikitext',
			formatversion: '2',
		}).done(function (spriteMod) {
			const cmdSrc = spriteMod.parse.wikitext
				.replace(
					"return result .. '[[Category:使用了Command模块已弃用功能的页面]]'",
					"mw.addWarning( 'CMDDEPRECATED:cmd=' .. slash .. table.concat( command, ' ' ):gsub( '&#32;', ' ' ) )\n\t\treturn result"
				)
				.replace(
					"\t\t\t\t\tif not section[i] and ( fullCommand or params[param] == '?' ) then\n\t\t\t\t\t\tdeprecated = true",
					"\t\t\t\t\tif not section[i] and ( fullCommand or params[param] == '?' ) then\nmw.addWarning( 'CMDDEPRECATED:reason=FULLCMD' )\n\t\t\t\t\t\tdeprecated = true"
				);
			const logPrefix = 'CMDDEPRECATED:';

			api.post({
				action: 'parse',
				page: config.wgPageName,
				prop: 'parsewarnings',
				templatesandboxtitle: 'Module:Command',
				templatesandboxtext: cmdSrc,
				templatesandboxcontentmodel: 'Scribunto',
				formatversion: 2,
			}).done(function (data) {
				const parsed = data.parse.parsewarnings
					.filter((c) => c.includes(logPrefix))
					.map((c) =>
						c.substring(c.indexOf(logPrefix) + logPrefix.length)
					);
				if (parsed.length == 0)
					return OO.ui.alert(
						'minecraft-wiki zh-cmd-deprecate-tracker failed to process page!'
					);
				let data = {};
				let cmds = [];
				for (const c of parsed) {
					const k = c.substring(0, c.indexOf('='));
					const v = c.substring(c.indexOf('=') + 1);
					data[k] = v;
					if (k == 'cmd') {
						cmds.push(data);
						data = {};
					}
				}

				let text = '已弃用的{{Template link|cmd}}用法：\n';
				for (const c of cmds) {
					text += `* <code>${
						c.cmd
					}</code><span class="mcwzh-cmd-deprecate-data">${JSON.stringify(
						c
					)}</span>\n`;
					if ('reason' in c) {
						text += `: 原因：${
							REASONS[c.reason] || `<code>${c.reason}</code>`
						}\n`;
					}
				}

				api.parse(text, {
					disablelimitreport: true,
					wrapoutputclass:
						'mcwzh-cmd-deprecate-tracker mw-message-box mw-content-' +
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
