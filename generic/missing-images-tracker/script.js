mw.loader
	.using(['mediawiki.util', 'mediawiki.api', 'oojs-ui-windows'])
	.done(function () {
		const config = mw.config.get([
			'wgWikiID',
			'wgCategories',
			'wgPageName',
			'wgUserLanguage',
		]);
		window.loadedMWMissingImgTracker = true;
		if (!(config.wgCategories.includes('含有受损文件链接的页面') || config.wgCategories.includes('Pages with broken file links')))
			// @todo: i18n
			return;

		mw.util.addCSS(
			'.xtex-mw-missing-images-tracker { background: none; padding: 0.5em; }'
		);

		const api = new mw.Api();

		api.get({
			action: 'parse',
			page: config.wgPageName,
			prop: 'images',
			formatversion: '2',
		}).done(function (parsePage) {
			let imgs = parsePage.parse.images;
			let missing = [];

			function resolveImages() {
				if (imgs.length == 0) {
					return showMissing();
				}
				let batch = [];
				for (let i = 0; i < 50 && imgs.length != 0; ++i) {
					batch.push('File:' + imgs.pop());
				}
				console.log('missing images tracker: fetching ', batch);

				api.post({
					action: 'query',
					titles: batch.join('|'),
					formatversion: 2,
				}).done(function (queryData) {
					const data = queryData.query.pages
						.filter((c) => 'missing' in c && c.missing)
						.map((c) => c.title);
					for (const c of data) {
						missing.push(c);
					}
					resolveImages();
				});
			}

			function showMissing() {
				let text = '缺少的图片：\n';
				if (missing.length == 0)
					text += '<无>\n';
				for (const c of missing) {
					text += `* <code>${c}</code>\n`;
				}

				api.parse(text, {
					disablelimitreport: true,
					wrapoutputclass:
						'xtex-mw-missing-images-tracker mw-message-box mw-content-' +
						($('#contentSub').attr('dir') || 'ltr'),
					uselang: config.wgUserLanguage,
				}).done(function (pt) {
					mw.hook('wikipage.content').fire(
						$(pt).appendTo('#bodyContent')
					);
				});
			}

			resolveImages();
		});
	});
