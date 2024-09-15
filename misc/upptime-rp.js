// SPDX-License-Identifier: Unlicense

// Usage: add the following configuration to .upptimerc.yml
// ```yaml
// status-website:
//   scripts:
//     - src: https://xtex.codeberg.page/gadgets/@main/misc/upptime-rp.js
// ```

(function () {
	const RP = 'https://rp.chitang.dev/';

	var fetch0 = fetch;
	window.fetch = (input, init) => fetch0(RP + input, init);

	const observer = new MutationObserver((mutationList, observer) => {
		/**
		 * @param {HTMLElement} node
		 */
		function patchNode(node) {
			const src = node.getAttribute('src');
			if (src && !src.startsWith(RP))
				node.setAttribute('src', RP + src);

			const style = node.getAttribute('style');
			if (style && style.includes('https://'))
				node.setAttribute('style', style.replace('https://', RP + 'https://'));
		}
		for (const mutation of mutationList) {
			patchNode(node.target);
			for (const node of mutation.addedNodes)
				patchNode(node);
		}
	});
	observer.observe(document.getElementById("sapper"), { subtree: true, attributes: false, childList: true });
})();
