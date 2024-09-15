// SPDX-License-Identifier: Unlicense

// Usage: add the following configuration to .upptimerc.yml
// ```yaml
// status-website:
//   scripts:
//     - src: https://xtex.codeberg.page/gadgets/@main/misc/upptime-rp.js
// ```

(function () {
	var originalFetch = fetch;
	window.fetch = (input, init) => originalFetch('https://rp.chitang.dev/' + input, init);
})();
