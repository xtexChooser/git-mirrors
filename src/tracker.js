if (!localStorage["xtex-home:stat:opt-out"]) {
	const el = document.getElementById("tracker-id");
	if (el != undefined) {
		console.log("Sending stats ping: " + el.value);
		var xhr = new XMLHttpRequest();
		xhr.open("GET", "/stats_ping/" + el.value, true);
		xhr.send(null);
	}
}

function tracker_optout() {
	localStorage["xtex-home:stat:opt-out"] = true;
	alert("We won't send any statistics information anymore.");
}

function tracker_optin() {
	delete localStorage["xtex-home:stat:opt-out"];
	alert("Opt-in succeeded.");
}
