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
	payload_config: {
		profile    : null, // The Profile name    (default: null)
		params_diff: null, // The ParamsDiff      (default: null)
		unthread   : true, // Enable unthreading  (default: true)
		read_cache : true, // Read from the cache (default: true)
		write_cache: true, // Write to the cache  (default: true)
		cache_delay: true, // Enable cache delay  (default: true)
	},
	debug: false
};

(() => {
	cleaned_elements = new WeakMap(); // Elements that have been cleaned (or errored) and the hrefs they were set to (or left at).
	queue = []; // Elements in order of soonest next task result.
	click_on_clean = null; // When an element is unclean when clicked, stop the click and put it here to click it once cleaned.

	// Set up the URL for the socket.
	let socket_url = new URL(window.config.instance);
	let config = {
		"context": {
			"source_host": window.location.hostname
		},
		...window.config.payload_config
	};
	socket_url.searchParams.append("config", JSON.stringify(config));

	// Send an element from the queue to Site, doing shortcuts and context as needed.
	function send_element(element) {

		if (element.href.startsWith("#")) {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring anchor:", element);
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

		if (window.config.debug) {
			console.debug("[URLC] Sending task", task, "for element", element);
		}

		queue.push(element);
		socket.send(task);
	}

	// Stops clicks on dirty links and remembers them to click on them later.
	function dirty_click_delayer(e) {
		if (socket.readyState == 1 && queue.indexOf(e.target) != -1) {
			if (window.config.debug) {
				console.debug("[URLC] Delaying click for unclean element:", e.target);
			}
			e.preventDefault();
			click_on_clean = e.target;
		}
	}

	// Attach the dirty click delayer.
	window.addEventListener("click", dirty_click_delayer);

	// Observing changes to href attributes.
	let attribute_observer = new MutationObserver(function(mutations) {
		for (let mutation of mutations) {
			send_element(mutation.target);
		}
	});

	// Observing changes to the node tree.
	let tree_observer = new MutationObserver(function(mutations) {
		for (let mutation of mutations) {
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

	// Make the socket.
	let socket = new WebSocket(socket_url);

	// When the socket opens, send all existing links.
	socket.addEventListener("open", function() {
		console.debug("[URLC] Opened socket to", socket_url.href);

		// Watch changes to any href attribute.
		attribute_observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ["href"],
			subtree: true
		});

		// Watch changes to the node tree.
		tree_observer.observe(document.documentElement, {
			subtree: true,
			childList: true
		});

		// Clean all existing links.
		for (element of document.links) {
			send_element(element);
		}
	});

	// When getting a message, apply the clean.
	socket.addEventListener("message", function(message) {
		if (typeof message.data === "string") {
			for (line of message.data.trimEnd().split("\n")) {
				let element = queue.shift();
				if (line.startsWith("-")) {
					console.error("[URLC] Got error", line, "for element", element);
					cleaned_elements.set(element, element.href);
				} else {
					if (window.config.debug) {
						console.debug("[URLC] Got success", line, "for element", element);
					}
					cleaned_elements.set(element, line);
					if (element.href != line) {
							element.href = line;
					}
				}

				// If the element was clicked when dirty (and thus had the click intercepted), click it.
				if (element == click_on_clean) {
					if (window.config.debug) {
						console.debug("[URLC] Redoing delayed click for now clean element:", element);
					}
					element.click();
					click_on_clean = null;
				}
			}
		}
	});

	// Print a message when the socket is closing.
	socket.addEventListener("close", function(e) {
		if (window.config.debug) {
			console.debug("[URLC] Closing socket:", e);
			attribute_observer.disconnect();
			tree_observer.disconnect();
			window.removeEventListener(dirty_click_delayer);
		}
	});

	// Print a message when the socket errors.
	socket.addEventListener("error", function(e) {
		if (window.config.debug) {
			console.error("[URLC] Socket error:", e);
			attribute_observer.disconnect();
			tree_observer.disconnect();
			window.removeEventListener(dirty_click_delayer);
		}
	});

	// Close the socket and disconnect the observer.
	window.addEventListener("beforeunload", () => {
		if (window.config.debug) {
			console.debug("[URLC] Doing beforeunload cleanup.");
		}
		socket.close();
	});
})();
