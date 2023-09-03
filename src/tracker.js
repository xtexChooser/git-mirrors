if (!localStorage["xtex-home:stat:opt-out"]) {
	const el = document.getElementById("tracker-id");
	if (el != undefined) {
		console.log("Sending stats ping: " + el.value);
		var xhr = new XMLHttpRequest();
		xhr.open("GET", "/stats_ping/" + el.value, true);
		xhr.send(null);
	}
}

(function () {
	const optout = document.getElementById("tracker_optout")
	if (optout == undefined) return;
	optout.onclick = function () {
		localStorage["xtex-home:stat:opt-out"] = true;
		alert("We won't send any statistics information anymore.");
	}

	const optin = document.getElementById("tracker_optin")
	optin.onclick = function () {
		delete localStorage["xtex-home:stat:opt-out"];
		alert("Opt-in succeeded.");
	}
})();
