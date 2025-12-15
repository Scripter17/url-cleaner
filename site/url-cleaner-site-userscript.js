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
	job_config: {
		password   : null, // The password        (default: null)
		profile    : null, // The Profile name    (default: null)
		params_diff: null, // The ParamsDiff      (default: null)
		unthread   : true, // Enable unthreading  (default: true)
		read_cache : true, // Read from the cache (default: true)
		write_cache: true, // Write to the cache  (default: true)
		cache_delay: true, // Enable cache delay  (default: true)
	},
	debug: false
};

function urlc_main() {
	console.log(`[URLC] URL Cleaner Site Userscript ${GM.info.script.version}
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
${GM.info.script.namespace}`);

	if (window.config.debug) {
		console.debug("[URLC] The config is", window.config);
	}

	// A WeakSet of elements that were cleaned but whose href attribute mutation hasn't been observed yet.
	// Elements removed from the document but not dropped (probably) have their cleanings added to the set but not noticed by the MutationObserver.
	// So having it be a FIFO queue wouln't work.
	let just_cleaned = new WeakSet();
	// A FIFO queue for elements sent to URL Cleaner Site whose results are not yet recieved.
	// Contains [WeakRef<HTMLElement>, String]s, where the string is the element's href at the time it was sent to URL Cleaner Site.
	let queue = [];
	// When an element is unclean when clicked, stop the click and put it here to click it once cleaned.
	// currently there doesn't seem to be a way to do the same for middle clicks.
	let click_on_clean = null;
	// `true` if the socket ever had an `open` event.
	// Used to provide extra debug info if the socket couldn't be opened.
	let socket_ever_opened = false;

	// Allows running the getter only once, improving performance.
	let hostname = window.location.hostname;

	// Set up the URL for the socket.
	let socket_url = new URL(window.config.instance);
	let config = {...window.config.job_config};
	config.context = {...config.context, "source_host": hostname};
	socket_url.searchParams.append("config", JSON.stringify(config));

	if (window.config.debug) {
		console.debug("[URLC] The WebSocket URL is", socket_url);
	}

	// Used to decide which per-site unmangling/shortcuts to do.
	let host_category;

	if (/(?:^|\.)furaffinity\.net\.?$/.test(hostname)) {
		host_category = "furaffinity";
	} else if (/(?:^|\.)x\.com\.?$/g         .test(hostname)) {
		host_category = "twitter";
	} else if (/(?:^|\.)allmylinks\.com\.?$/g.test(hostname)) {
		host_category = "allmylinks";
	} else if (/(?:^|\.)bsky\.app\.?$/g      .test(hostname)) {
		host_category = "bluesky";
	} else if (/(?:^|\.)saucenao\.com\.?$/g  .test(hostname)) {
		host_category = "saucenao";
	} else if (/(?:^|\.)duckduckgo\.com\.?$/g.test(hostname)) {
		host_category = "duckduckgo";
	}

	if (window.config.debug) {
		console.debug("[URLC] The host category is", host_category);
	}

	// If applicable, send an element's task to URL Cleaner Site and add it to the queue.
	function urlc_queue_element(element) {

		if (window.config.debug) {
			console.debug("[URLC] Maybe queueing", element);
		}

		// Sometimes the code that get elements to clean finds elements that can't and/or shouldn't be cleaned.
		// Due to javascript's abysmal type system, not manually discovering and detecting these cases can result in the userscript ending up in invalid states.

		// Mimic document.links.
		// Yeah turns out `<area>` is an element that exists. Who knew?
		if (element.tagName !== "A" && element.tagName !== "AREA") {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring non-link element", element);
			}
			return;
		}

		// Allows running the getter only once, improving performance.
		let href = element.href;

		// If the element is null/undefined/otherwise not a string, just ignore it.
		// This comes up in `use` elements in SVG... markup?
		if (typeof href !== "string") {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring non-string href", element);
			}
			return;
		}

		// Ignore empty anchors to break fewer websites.
		// Some websites use href="#" to make a link element look like a link but work like a button.
		// In any anchors that would get cleaned (some websites put tracking parameters in them) are unlikely to break stuff when cleaned.
		if (href.includes("#") && element.getAttribute("href") === "#") {
			if (window.config.debug) {
				console.debug("[URLC] Ignoring anchor link", element);
			}
			return;
		}

		// Some websites have links that are best cleaned using details from places other than just their URL.

		// The variable the task is eventually put in.
		let task;

		if (host_category === "furaffinity" && element.matches(".user-contact-user-info a")) {
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
		} else if (host_category === "twitter" && href.startsWith("https://t.co/") && element.innerText.startsWith("http")) {
			// Even when a link in a tweet is shown as `https://example.com/really-long-...`, twitter still puts the full `really-long-url` in the HTML.
			// By getting it from there, we can skip an HTTP request to t.co.
			task = element.childNodes[0].innerText + element.childNodes[1].textContent + (element.childNodes[2]?.innerText ?? "");
		} else if (host_category === "allmylinks" && element.pathname === "/link/out" && element.title.startsWith("http")) {
			// Same idea as above.
			task = element.title;
		} else if (host_category === "bluesky" && href.startsWith("/profile/did:plc:") && element.innerText.startsWith("@")) {
			// Replaces `/profile/did:plc:` URLs with `/profile/example.com`, as it should be.
			task = href.replace(/did:plc:[^/]+/g, element.innerText.replace("@", ""))
		} else if (host_category === "saucenao" && /^https:\/\/(?:www\.)?(?:x|twitter)\.com\//.test(href)) {
			// Replaces `/i/web/1234` and `/i/user/1234` in twitter links with their normal forms.
			task = href.replace(/i\/web|i\/user\/\d+/g, element.parentElement.querySelector("[href*='/i/user/']").innerHTML.replace("@", ""));
		} else if (host_category === "duckduckgo" && window.location.pathname == "/" && /[?&]q=/g.test(window.location.search) && element.matches("[data-testid^=result-]")) {
			// % incorrectly gets percent encded.
			task = element.closest("li").querySelector("a p").textContent.replaceAll(/\s\u203a\s/g, "/");
		} else {
			// For any other case, assume the element's href is sufficient.
			task = href;
		}

		if (window.config.debug) {
			console.debug("[URLC] Sending task", task, "for element", element);
		}

		socket.send(task);
		queue.push([new WeakRef(element), href]);
	}

	// Delay clicks on dirty links until they're cleaned.

	function urlc_dirty_click_delayer(e) {
		if (socket.readyState === 1 && queue.includes(e.target)) {
			if (window.config.debug) {
				console.debug("[URLC] Delaying click for dirty element", e);
			}
			e.preventDefault();
			click_on_clean = e.target;
		}
	}
	window.addEventListener("click", urlc_dirty_click_delayer);

	// Listen for changes to changes to any element's href attribute.

	function urlc_mutation_href(mutations) {
		if (window.config.debug) {
			console.debug("[URLC] Attribute mutations", mutations);
		}
		for (let mutation of mutations) {
			if (!just_cleaned.delete(mutation.target)) {
				urlc_queue_element(mutation.target);
			}
		}
	}
	let href_attribute_observer = new MutationObserver(urlc_mutation_href);

	// Listen for changes to the node tree.

	function urlc_tree_mutation(mutations) {
		if (window.config.debug) {
			console.debug("[URLC] Tree mutations", mutations);
		}
		for (let mutation of mutations) {
			for (let node of mutation.addedNodes) {
				if (node.nodeType === 1) {
					if (node.href) {
						urlc_queue_element(element);
					}
					for (let element of node.querySelectorAll("[href]")) {
						urlc_queue_element(element);
					}
				}
			}
		}
	}
	let child_list_observer = new MutationObserver(urlc_tree_mutation);

	// Make the socket.

	let socket = new WebSocket(socket_url);

	// When the socket is open, start cleaning.

	function urlc_socket_open() {
		if (window.config.debug) {
			console.debug("[URLC] Opened socket to", socket_url.href);
		}

		socket_ever_opened = true;

		// The observers must be attached after the socket is open.

		// Watch changes to any href attribute.
		href_attribute_observer.observe(document.documentElement, {
			attributeFilter: ["href"],
			subtree: true
		});

		// Watch changes to the node tree.
		child_list_observer.observe(document.documentElement, {
			childList: true,
			subtree: true
		});

		// Clean all existing links.
		for (element of document.links) {
			urlc_queue_element(element);
		}
	}
	socket.addEventListener("open", urlc_socket_open);

	// The handler for the socket's message event.
	function urlc_socket_message(message) {
		// Ignore pings, pongs, etc. and get only return frames, which are always strings.
		if (typeof message.data === "string") {
			for (line of message.data.split(/\r\n|\n/g)) {
				// Ignore empty lines.
				if (line === "") {continue;}

				// Ignore lines of unknown format.
				if (!/^[-a-zA-Z]/g.test(line)) {
					console.warn("[URLC] Unknown output line:", line);
					continue;
				}

				// Get the element this line is for.
				var [element, old_href] = queue.shift();

				element = element.deref();

				// The element was garbage collected and thus doesn't need to be cleaned.
				if (element === undefined) {continue;}

				if (line.startsWith("-")) {
					// Lines that start with `-` are errors.
					console.error("[URLC] Got error", line.replace("-", ""), "for element", element);
				} else {
					// Lines that don't start with `-` are successes.
					if (window.config.debug) {
						console.debug("[URLC] Got success", line, "for element", element);
					}
					// If the URL is unchanged, don't risk breaking a website's internal state for no reason.
					if (old_href !== line) {
							just_cleaned.add(element);
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
	}
	socket.addEventListener("message", urlc_socket_message);

	// When the socket is closing, print a message and clean up.

	function urlc_socket_close(e) {
		if (window.config.debug) {
			console.debug("[URLC] Closing socket", e);
		}
		href_attribute_observer.disconnect();
		child_list_observer.disconnect();
		window.removeEventListener("click", urlc_dirty_click_delayer);
	}
	socket.addEventListener("close", urlc_socket_close);

	// When the socket errors, print a message and clean up.

	function urlc_socket_error(e) {
		console.error("[URLC] Socket error", e);
		if (!socket_ever_opened) {
			console.error(`[URLC] It seems the socket couldn't be opened.
The server might be unreachable, you might have bad credentials, or if you're using TLS/HTTPS, your OS/browser might not trust your certificate.
For information on how to install/trust your certificate, see the docs: https://github.com/Scripter17/url-cleaner/blob/main/site/server.md#installing-the-certificate`);
		}
		href_attribute_observer.disconnect();
		child_list_observer.disconnect();
		window.removeEventListener("click", urlc_dirty_click_delayer);
	}
	socket.addEventListener("error", urlc_socket_error);

	// When leaving a webpage, do cleanup.

	function urlc_beforeunload(e) {
		if (window.config.debug) {
			console.debug("[URLC] Doing beforeunload cleanup", e);
		}
		socket.close();
	}
	window.addEventListener("beforeunload", urlc_beforeunload);
}

urlc_main();
