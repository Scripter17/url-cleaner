// ==UserScript==
// @name         URL Cleaner Site Userscript
// @description  The userscript that comes with URL Cleaner Site.
// @author       Scripter17@Github.com
// @namespace    https://github.com/Scripter17/url-cleaner
// @copyright    AGPL-3.0-or-later
// @version      0.12.0-beta
// @match        http://*/*
// @match        https://*/*
// @grant        GM.xmlHttpRequest
// @connect      localhost
// ==/UserScript==

window.config = {
	instance   : "ws://localhost:9149/clean_ws", // Login info can be added with `username:password@` before the `localhost`.
	profile    : null, // The Profile name    (default: null)
	params_diff: null, // The ParamsDiff      (default: null)
	read_cache : true, // Read from the cache (default: true)
	write_cache: true, // Write to the cache  (default: true)
	cache_delay: true, // Enable cache delay  (default: true)
	unthread   : true, // Enable unthreading  (default: true)
	extra_debug: false
};

(() => {
	cleaned_elements = new WeakMap(); // Elements that have been cleaned (or errored) and the hrefs they were set to.
	queue = []; // Elements in order of soonest next task result.

	// Send an element from the queue to Site, doing shortcuts and context as needed.
	function send_element(element) {

		if (element.href.startsWith("#")) {
			if (window.config.extra_debug) {
				console.debug("[URLC] Ignoring element", element, "because it's URL is just an anchor.");
			}
			return;
		}

		let task = element.href;

		if (/(^|\.)furaffinity\.net$/.test(window.location.hostname) && element.matches(".user-contact-user-info a")) {
			// Allows unmangling contact info links.
			if (element.href == "javascript:void(0);") {
				task = "https://" + element.innerText;
			} else {
				// Some contact info fields let invalid inputs result in invalid URLs, which URL Cleaner can't accept.
				// Since the unmangling for this doesn't touch the actual URL, just replace it with a dummy.
				task = JSON.stringify({
					url: "https://example.com/url_cleaner_dummy",
					context: {
						vars: {
							contact_info_site_name: element.parentElement.querySelector("strong").innerHTML,
							link_text: element.innerText
						}
					}
				});
			}
		} else if (/(^|\.)x\.com$/.test(window.location.hostname) && element.href.startsWith("https://t.co/") && element.innerText.startsWith("http")) {
			// On twitter, links in tweets/bios/whatever show the entire URL when you hover over them for a moemnt.
			// This lets us skip the HTTP request to t.co for the vast majority of links on twitter.
			task = element.childNodes[0].innerText + element.childNodes[1].textContent + (element.childNodes[2]?.innerText ?? "");
		} else if (/(^|\.)allmylinks\.com$/.test(window.location.hostname) && element.pathname=="/link/out" && element.title.startsWith("http")) {
			// Same shortcut thing as the twitter stuff above.
			task = element.title;
		} else if (/(^|\.)bsky\.app$/.test(window.location.hostname) && element.getAttribute("href").startsWith("/profile/did:plc:") && element.innerText.startsWith("@")) {
			// Allows replacing `/profile/did:plc:` URLs with `/profile/example.com`, as it should be.
			task = eleement.href.replace(/did:plc:[^/]+/g, element.innerText.replace("@", ""))
		} else if (/(^|\.)saucenao\.com$/.test(window.location.hostname) && /^https:\/\/(www\.)?(x|twitter)\.com\//.test(element.href)) {
			// Fixes twitter URLs returned by SauceNAO.
			task = element.href.replace(/i\/web|i\/user\/\d+/g, element.parentElement.querySelector("[href*='/i/user/']").innerHTML.replace("@", ""));
		}

		if (window.config.extra_debug) {
			console.debug("[URLC] Sending task", task, "for element", element);
		}

		queue.push(element);
		socket.send(task);
	}

	// Set up the URL for the socket.
	let socket_url = new URL(window.config.instance);
	socket_url.searchParams.append("context", JSON.stringify({"source_host": window.location.hostname}));
	if (window.config.profile     !== null ) {socket_url.searchParams.append("profile"    , JSON.stringify(window.config.profile    ));}
	if (window.config.params_diff !== null ) {socket_url.searchParams.append("params_diff", JSON.stringify(window.config.params_diff));}
	if (window.config.read_cache  !== true ) {socket_url.searchParams.append("read_cache" , JSON.stringify(window.config.read_cache ));}
	if (window.config.write_cache !== true ) {socket_url.searchParams.append("write_cache", JSON.stringify(window.config.write_cache));}
	if (window.config.cache_delay !== false) {socket_url.searchParams.append("cache_delay", JSON.stringify(window.config.cache_delay));}
	if (window.config.unthread    !== false) {socket_url.searchParams.append("unthread"   , JSON.stringify(window.config.unthread   ));}

	// Make the socket.
	let socket = new WebSocket(socket_url);

	// Observe all changes to the page.
	let observer = new MutationObserver(function(mutations) {
		mutations.forEach(function(mutation) {
			if (mutation.type == "attributes") {
				send_element(mutation.target);
			} else if (mutation.type == "childList") {
				for (let node of mutation.addedNodes) {
					if (node.nodeType == 1) {
						if (node.href) {
							send_element(node);
						}
						for (element of node.querySelectorAll("[href]")) {
							send_element(element);
						}
					}
				}
			}
		});
	});

	// When the socket opens, send all existing links.
	socket.addEventListener("open", function() {
		console.debug("[URLC] Opened socket to", socket_url.href);

		// Watch changes to any href attribute.
		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["href"],
			subtree: true
		});

		// Watch changes to the node tree.
		observer.observe(document.documentElement, {
			subtree: true,
			childList: true
		});

		for (element of document.links) {
			send_element(element);
		}
	});

	// WHen getting a message, apply the clean.
	socket.addEventListener("message", function(message) {
		let [status, payload] = message.data.split("\t");
		let element = queue.shift();
		if (window.config.extra_debug) {
			console.debug("[URLC] Got", status, "with", payload, "for element", element);
		}
		if (status == "Ok") {
			cleaned_elements.set(element, payload);
			element.href = payload;
		} else if (status == "Err") {
			cleaned_elements.set(element, element.href);
		}
	});

	// Print a message when the socket is closing.
	socket.addEventListener("close", function() {
		console.debug("[URLC] Closing socket.");
	});

	socket.addEventListener("error", function(e) {
		console.error("[URLC] Socket error:", e);
	})

	// Close the socket and disconnect the observer.
	window.addEventListener("beforeunload", () => {
		socket.close();
		observer.disconnect();
	});
})();
