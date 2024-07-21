// ==UserScript==
// @name         PostgreSQL Doc Title Shortener
// @namespace    https://xtexx.eu.org/
// @version      2024-07-21
// @description  Make the title of PostgreSQL Documentation shorter!
// @author       xtex
// @match        https://www.postgresql.org/docs/*
// @icon         https://www.postgresql.org/favicon.ico
// @grant        none
// @run-at document-body
// @noframes
// @supportURL   https://codeberg.org/xtex/gadgets/issues
// ==/UserScript==

(function () {
	'use strict';

	if (document.title.startsWith('PostgreSQL: Documentation: ')) {
		document.title = document.title.replace(
			/^PostgreSQL: Documentation: \d+: /,
			'PGDoc: '
		);
	}
})();
