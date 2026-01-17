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

function urlc_main() {
	let config = {
		// If using TLS, change `ws://` to `wss://`.
		// If using a remote server, change both this `localhost` and the `@connect localhost` above to the same host.
		instance: "ws://localhost:9149",
		job_config: {
			context: {
				source_host: window.location.hostname // Used for per-site processing.
			},
			// password   : null, // The password        (Site default: null                           )
			// profile    : null, // The Profile name    (Site default: null                           )
			// params_diff: null, // The ParamsDiff      (Site default: null                           )
			   unthread   : true, // Enable unthreading  (Site default: false, Userscript default: true)
			// read_cache : true, // Read from the cache (Site default: true                           )
			// write_cache: true, // Write to the cache  (Site default: true                           )
			   cache_delay: true, // Enable cache delay  (Site default: false, Userscript default: true)
		},
		debug: false
	};

	// A map of elements to the last URL they were cleaned to.
	// This allows for ignoring the mutation observed by normal cleaning.
	let cleans = new WeakMap();
	// Tasks are queued then sent all at once when the buffer is "full" or there's a moment with no new tasks.
	// This dramatically reduces the overhead of WebSockets by using far fewer messages.
	let tasks = "";
	// When the first task is queued, a timer is set to send the queue even if it isn't "full" yet and the timer's ID is stored here.
	// When the queue gets "full", this is used to stop the timer.
	let send_tasks_timeout_id = null;
	// A queue of [WeakRef<HTMLElement>, String]s representing each element sent to URL Cleaner Site.
	// The String is the element's href at the time it was sent. This allows not applying redundant cleans.
	let queue = [];
	// If an element is clicked while in the queue, the click is blocked and redone once the element has been cleaned.
	// This thwarts websites that dirty links as you click them. Unless you middle click. There seems to be no way to handle that.
	let reclick_once_clean = null;
	// If the socket ever actually opened, remember it so redundant debug info isn't logged.
	let socket_ever_opened = false;

	// Used to allow URL Cleaner Site to do cleanings that only apply on certain websites.
	let hostname = window.location.hostname;
	// Used by urlc_queue_element to do shortcuts such as avoiding most redirects on twitter.
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

	// Set up the URL for the socket.
	let socket_url = new URL(config.instance);
	socket_url.pathname = "/clean";
	socket_url.searchParams.append("config", JSON.stringify(config.job_config));

	console.log(`[URLC] URL Cleaner Site Userscript ${GM.info.script.version}
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
${GM.info.script.namespace}`,
			"\nConfig:"       , config,
			"\nSocket URL:"   , socket_url,
			"\nHost:"         , hostname,
			"\nHost category:", host_category,
			"\nCleans:"       , cleans,
			"\nQueue:"        , queue
	);

	// If applicable, send an element's task to URL Cleaner Site and add it to the queue.
	function urlc_queue_element(element) {

		if (config.debug) {
			console.debug("[URLC] Maybe queueing", element);
		}

		// Filter out elements that can't/shouldn't be cleaned.

		// Mimic document.links.
		// Yeah turns out `<area>` is an element that exists. Who knew?
		if (!(element?.tagName === "A" || element?.tagName === "AREA")) {
			if (config.debug) {
				console.debug("[URLC] Ignoring: Not a link.");
			}
			return;
		}

		// `element.href` gets an absolute URL, which is not what we need and also expensive.
		let hrefattr = element.getAttribute("href");

		// If the element doesn't have a href there isn't a URL to clean.
		if (hrefattr === null) {
			if (config.debug) {
				console.debug("[URLC] Ignoring: No href.");
			}
			return;
		}

		// Plain `#` hrefs are used to turn links into buttons.
		// Removing the empty fragment, while correct outside of a webpage, breaks stuff in webpages.
		if (hrefattr === "#") {
			if (config.debug) {
				console.debug("[URLC] Ignoring: Href is \"#\".");
			}
			return;
		}

		// If an element has been cleaned and hasn't been changed since its href attribute will be contained in `cleans`.
		if (cleans.get(element) === hrefattr) {
			if (config.debug) {
				console.debug("[URLC] Ignoring: Already clean.")
			}
			return;
		}

		// If `href` is absolute, use it instead of making `element.href` compute it.
		let href;
		if (/^https?:\/\//gi.test(hrefattr)) {
			href = hrefattr;
		} else {
			href = element.href;
		}

		// Some websites have links that are best cleaned using details from places other than just their URL.

		let task;

		if (host_category === "furaffinity" && element.matches(".user-contact-user-info a")) {
			// If a contact info field has more of the URL than expected, such as `https://x.com/user` instead of just `user`,
			// the URL of the link is incoherent and very hard to unmangle.
			// For that the bundled cleaner parses the link's text and extracts just the expected part.
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

		if (config.debug) {
			console.debug("[URLC] Sending task", task, "for element", element);
		}

		queue.push([new WeakRef(element), href]);

		// If the buffer isn't "full" within 10ms, just send it.
		if (tasks.length === 0) {
			send_tasks_timeout_id = setTimeout(urlc_send_tasks, 10);
		}
		tasks += task;
		tasks += "\n";
		// If the buffer is "full", send it.
		if (tasks.length >= 65536) {
			urlc_send_tasks();
		}
	}

	// Actually send the tasks.

	function urlc_send_tasks() {
		clearTimeout(send_tasks_timeout_id);
		send_tasks_timeout_id = null;
		socket.send(tasks);
		tasks = "";
	}

	// Delay clicks on dirty links until they're cleaned.

	function urlc_dirty_click_delayer(e) {
		if (socket.readyState === 1) {
			for (let [queued_element, _] of queue) {
				if (queued_element.deref() == e.target) {
					if (config.debug) {
						console.debug("[URLC] Delaying click for dirty element", e);
					}
					e.preventDefault();
					reclick_once_clean = e.target;
					return;
				}
			}
		}
	}
	window.addEventListener("click", urlc_dirty_click_delayer);

	// Listen for changes to changes to any element's href attribute.

	function urlc_mutation_href(mutations) {
		if (config.debug) {
			console.debug("[URLC] Attribute mutations", mutations);
		}
		for (let mutation of mutations) {
			urlc_queue_element(mutation.target);
		}
	}
	let href_attribute_observer = new MutationObserver(urlc_mutation_href);

	// Listen for changes to the node tree.

	function urlc_tree_mutation(mutations) {
		if (config.debug) {
			console.debug("[URLC] Tree mutations", mutations);
		}
		for (let mutation of mutations) {
			for (let node of mutation.addedNodes) {
				if (node.nodeType === 1) {
					urlc_queue_element(node);
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
		if (config.debug) {
			console.debug("[URLC] Opened socket to", socket_url.href);
		}

		socket_ever_opened = true;

		// Attempting to send tasks before the socket is open results in an error,
		// so the MutationObservers have to be attached only when the socket is open.

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

				// If the element was garbage collected it can't be cleaned.
				if (element === undefined) {
					continue;
				}

				if (line.startsWith("-")) {
					// Lines that start with `-` are errors.
					console.error("[URLC] Got error", line.replace("-", ""), "for element", element);
				} else {
					// Lines that don't start with `-` are successes.
					if (config.debug) {
						console.debug("[URLC] Got success", line, "for element", element);
					}
					// If the URL is unchanged, don't risk breaking a website's internal state for no reason.
					if (old_href !== line) {
						cleans.set(element, line);
						element.href = line;
					}
				}

				// If the element was clicked when dirty (and thus had the click intercepted), click it.
				// See the function `dirty_click_delayer` for details.
				if (element === reclick_once_clean) {
					if (config.debug) {
						console.debug("[URLC] Redoing delayed click for now clean element", element);
					}
					element.click();
					reclick_once_clean = null;
				}
			}
		}
	}
	socket.addEventListener("message", urlc_socket_message);

	// When the socket is closing, print a message and clean up.

	function urlc_socket_close(e) {
		console.debug("[URLC] Socket closed", e);
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
		console.debug("[URLC] Doing beforeunload cleanup", e);
		socket.close();
	}
	window.addEventListener("beforeunload", urlc_beforeunload);
}

urlc_main();
