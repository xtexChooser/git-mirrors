mw.loader.load('https://wikiplus-app.com/Main.js');
mw.loader.load('https://xtexx.ml/scripts/mw/auto_minor.js');

// Fandom
if (location.hostname.endsWith("fandom.com")) {
    importArticles({
        type: 'script',
        articles: [
            'u:dev:MediaWiki:NoImageLightbox/code.js',
            'u:dev:MediaWiki:View_Source/code.js',
            'u:dev:MediaWiki:LinkPreview/code.js',
            'u:dev:MediaWiki:MassEdit/code.js',
            'u:dev:MediaWiki:View_Raw/code.js',
            'u:dev:MediaWiki:UserUnusedFiles/code.js',
            'u:dev:MediaWiki:HTML5AudioPlayer/code.js',
        ]
    });
}
