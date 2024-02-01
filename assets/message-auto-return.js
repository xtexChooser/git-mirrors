(function() {
	setTimeout(function () {
		history.back();
	}, 3000);
	let el = document.querySelector("#message #auto-redirect-message");
	el.innerHTML = el.getAttribute("data-text");
})();
