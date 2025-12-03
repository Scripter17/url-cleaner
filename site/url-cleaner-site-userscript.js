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
	instance: "ws://localhost:9149/clean_ws", // Make sure to keep synced with the above `@connect` host.
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
	console.log(`[URLC] URL Cleaner Site Userscript ${GM.info.script.version}
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
${GM.info.script.namespace}`);

	if (window.config.debug) {
		console.debug("[URLC] The config is", window.config);
	}

	// A map of elements to the URL they were at when last handled.
	// Either the cleaned URL or, if cleaning returned an error, the URL it was at beforehand.
	// Used to not immediately re-queue elements because a MutationObserver saw it get cleaned.
	let handled = new WeakMap();
	// The list of elements currently being cleaned, in the order they were sent to URL Cleaner Site.
	let queue = [];
	// When an element is unclean when clicked, stop the click and put it here to click it once cleaned.
	// currently there doesn't seem to be a way to do the same for middle clicks.
	// Maybe should be a queue.
	let click_on_clean = null;
	// `true` if the socket ever had an `open` event.
	// Used to provide extra debug info if the socket couldn't be opened.
	let socket_ever_opened = false;

	// Set up the URL for the socket.
	let socket_url = new URL(window.config.instance);
	let config = {...window.config.payload_config};
	config.context = {...config.context, "source_host": window.location.hostname};
	socket_url.searchParams.append("config", JSON.stringify(config));

	if (window.config.debug) {
		console.debug("[URLC] The WebSocket URL is", socket_url);
	}

	// If applicable, add an element to the queue and send its task to URL Cleaner Site.
	function queue_element(element) {
		// If it's a Node that isn't an HTMLElement, ignore it.
		// Comes up when MutationObserver sees some other type of node (like a text node) was added.
		if (element.nodeType !== 1) {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring non-element node", element);
			}
			return;
		}

		// Mimic document.links.
		// Yeah turns out `<area>` is an element that exists. Who knew?
		if (element.tagName !== "A" && element.tagName !== "AREA") {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring non-link element", element);
			}
			return;
		}

		// If the element is null/undefined or somehow not a string, just ignore it.
		// Fun fact: `use` elements have hrefs that aren't strings.
		if (typeof element.href !== "string") {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring non-string href", element);
			}
			return;
		}

		// Ignore anchors.
		// TODO: In theory, this could filter out legitimately unclean URLs. I need to fix that.
		if (element.getAttribute("href").startsWith("#")) {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring anchor link", element);
			}
			return;
		}

		// Ignores elements a MutationObserver just saw get cleaned.
		if (element.href === handled.get(element)) {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring already handled link", element);
			}
			return;
		}

		// Some websites have links that are best cleaned using details from places other than just their URL.

		let task = element.href;

		if (/(^|\.)furaffinity\.net\.?$/.test(window.location.hostname) && element.matches(".user-contact-user-info a")) {
			// If a contact info field has more of the URL than expected, such as `https://x.com/user` instead of just `user`,
			// the URL of the link is incoherent and very hard to unmangle.
			// For that the bundled cleaner just parses the link's text and extracts just the expected part.
			task = JSON.stringify({
				url: "https://example.com/", // The URL might be invalid so we need a dummy value.
				context: {
					vars: {
						unmangle_mode: "contact_info",
						site: element.parentElement.querySelector("strong").innerHTML.toLowerCase(),
						text: element.innerText
					}
				}
			});
		} else if (/(^|\.)x\.com\.?$/.test(window.location.hostname) && element.href.startsWith("https://t.co/") && element.innerText.startsWith("http")) {
			// Even when a link in a tweet is shown as `https://example.com/really-long-...`, twitter still puts the full `really-long-url` in the HTML.
			// By getting it from there, we can skip an HTTP request to t.co.

			task = element.childNodes[0].innerText + element.childNodes[1].textContent + (element.childNodes[2]?.innerText ?? "");
		} else if (/(^|\.)allmylinks\.com\.?$/.test(window.location.hostname) && element.pathname === "/link/out" && element.title.startsWith("http")) {
			// Same idea as above.

			task = element.title;
		} else if (/(^|\.)bsky\.app\.?$/.test(window.location.hostname) && element.href.startsWith("/profile/did:plc:") && element.innerText.startsWith("@")) {
			// Replaces `/profile/did:plc:` URLs with `/profile/example.com`, as it should be.

			task = eleement.href.replace(/did:plc:[^/]+/g, element.innerText.replace("@", ""))
		} else if (/(^|\.)saucenao\.com\.?$/.test(window.location.hostname) && /^https:\/\/(www\.)?(x|twitter)\.com\//.test(element.href)) {
			// Replaces `/i/web/1234` and `/i/user/1234` in twitter links with their normal forms.

			task = element.href.replace(/i\/web|i\/user\/\d+/g, element.parentElement.querySelector("[href*='/i/user/']").innerHTML.replace("@", ""));
		}

		if (window.config.debug) {
			console.debug("[URLC] Sending task", task, "for element", element);
		}

		queue.push(element);
		socket.send(task);
	}

	// When an element is clicked while in the queue (and the socket is open), block clicks until it's been cleaned.
	function dirty_click_delayer(e) {
		if (socket.readyState === 1 && queue.indexOf(e.target) !== -1) {
			if (window.config.debug) {
				console.debug("[URLC] Delaying click for unclean element", e);
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
			queue_element(mutation.target);
		}
	});

	// Observing changes to the node tree.
	let tree_observer = new MutationObserver(function(mutations) {
		for (let mutation of mutations) {
			for (let node of mutation.addedNodes) {
				// More thorough filtering is done inside `queue_element`.
				if (node.nodeType === 1) {
					if (node.href) {
						queue_element(node);
					}
					for (element of node.querySelectorAll("[href]")) {
						queue_element(element);
					}
				}
			}
		}
	});

	// Make the socket.
	let socket = new WebSocket(socket_url);

	// When the socket opens, set up observers and queue existing links.
	socket.addEventListener("open", function() {
		if (window.config.debug) {
			console.debug("[URLC] Opened socket to", socket_url.href);
		}

		socket_ever_opened = true;

		// The observers must be attached after the socket is open.

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
			queue_element(element);
		}
	});

	// When getting a message, apply the clean.
	socket.addEventListener("message", function(message) {
		// Ignore pings, pongs, etc. and get only return frames, which are always strings.
		if (typeof message.data === "string") {
			let lines = message.data.split("\n");
			if (lines[lines.length - 1] === "") {lines.pop();}

			// Gotta remove the trailing newline.
			for (line of lines) {
				// Get the element this line is for.
				let element = queue.shift();

				if (line.startsWith("-")) {
					// Lines that start with `-` are errors.
					console.error("[URLC] Got error", line, "for element", element);
					handled.set(element, element.href);
				} else {
					// Lines that don't start with `-` are successes.
					if (window.config.debug) {
						console.debug("[URLC] Got success", line, "for element", element);
					}
					handled.set(element, line);
					// If the URL is unchanged, don't risk breaking a website's internal state for no reason.
					if (element.href !== line) {
							element.href = line;
					}
				}

				// If the element was clicked when dirty (and thus had the click intercepted), click it.
				// See the function `dirty_click_delayer` for details.
				if (element === click_on_clean) {
					if (window.config.debug) {
						console.debug("[URLC] Redoing delayed click for now clean element", element);
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
			console.debug("[URLC] Closing socket", e);
		}
		attribute_observer.disconnect();
		tree_observer.disconnect();
		window.removeEventListener("click", dirty_click_delayer);
	});

	// Print a message when the socket errors.
	socket.addEventListener("error", function(e) {
		console.error("[URLC] Socket error", e);
		if (!socket_ever_opened) {
			console.error(`[URLC] It seems the socket couldn't be opened.
The server might be unreachable, you might have bad credentials, or if you're using TLS/HTTPS, your OS/browser might not trust your certificate.
For information on how to install/trust your certificate, see the docs: https://github.com/Scripter17/url-cleaner/blob/main/site/server.md#installing-the-certificate`);
		}
		attribute_observer.disconnect();
		tree_observer.disconnect();
		window.removeEventListener("click", dirty_click_delayer);
	});

	// When leaving a webpage, do cleanup like closing the socket.
	window.addEventListener("beforeunload", (e) => {
		if (window.config.debug) {
			console.debug("[URLC] Doing beforeunload cleanup", e);
		}
		socket.close();
	});
})();
