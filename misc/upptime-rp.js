// SPDX-License-Identifier: Unlicense

(function () {
	var originalFetch = fetch;
	window.fetch = (input, init) => originalFetch('https://rp.chitang.dev/' + input, init);
}();
