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

urlc_main();

function urlc_main() {
	console.log(`[URLC] URL Cleaner Site Userscript ${GM.info.script.version}
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
${GM.info.script.namespace}`);

	let config = {
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
		}
	};

	let socket_url = new URL(config.instance);
	socket_url.pathname = "/clean";
	socket_url.searchParams.append("config", JSON.stringify(config.job_config));

	// The WebSocket.
	let socket = null;
	// Map of elements to the last URL they were assigned.
	// Used to avoid unnecessary recleans.
	let cache = new WeakMap();

	// Vec<(WeakRef<Element>, String)>.
	let queue = [];
	// A buffer used to send tasks in batches.
	let tasks = "";
	// The ID of the 10ms timeout to send tasks.
	let send_timeout = null;
	// If an element is clicked while in the queue, cacnel the click, store it here, then reclick it once clean.
	let reclick_once_clean = null;

	let href_mutation_observer = new MutationObserver(urlc_href_mutation);
	let tree_mutation_observer = new MutationObserver(urlc_tree_mutation);

	let hostname = window.location.hostname;
	let host_category = null;

       if (/(?:^|\.)furaffinity\.net\.?$/g.test(hostname)) {host_category = "furaffinity";}
	else if (/(?:^|\.)x\.com\.?$/g          .test(hostname)) {host_category = "twitter"    ;}
	else if (/(?:^|\.)duckduckgo\.com\.?$/g .test(hostname)) {host_category = "duckduckgo" ;}

	urlc_make_socket();

	// Clear the state and make a socket.
	// Called again whenever the previous socket exits with an erorr.
	function urlc_make_socket() {
		queue = [];
		tasks = "";
		clearTimeout(send_timeout);
		send_timeout = null;
		reclick_once_clean = null;

		socket = new WebSocket(socket_url);
		socket.addEventListener("open"   , urlc_socket_open);
		socket.addEventListener("message", urlc_socket_message);
		socket.addEventListener("close"  , urlc_socket_close);
		socket.addEventListener("error"  , urlc_socket_error);
	}

	// Attach listeners and clean existing links.
	function urlc_socket_open() {
		console.log("[URLC] Socket opened.");

		href_mutation_observer.observe(document.documentElement, {
			attributeFilter: ["href"],
			subtree: true
		});

		tree_mutation_observer.observe(document.documentElement, {
			childList: true,
			subtree: true
		});

		window.addEventListener("click", urlc_onclick);

		for (element of document.links) {
			urlc_queue_element(element);
		}
	}

	// Handle response messages.
	function urlc_socket_message(message) {
		for (line of message.data.split("\n")) {
			var [element, old_href] = queue.shift();

			element = element.deref();

			// If the element was garbage collected it can't be cleaned.
			if (element === undefined) {
				continue;
			}

			if (line.startsWith("-")) {
				console.error("[URLC] Got error", line.replace("-", ""), "for element", element);
			} else if (line !== old_href) {
				cache.set(element, line);
				element.href = line;
			}

			if (reclick_once_clean !== null && element === reclick_once_clean.deref()) {
				element.click();
				reclick_once_clean = null;
			}
		}
	}

	// Disconnect listeners and, if the socket closed with an error, try to reconnect.
	function urlc_socket_close(e) {
		console.log("[URLC] Socket closed with code", e.code);

		href_mutation_observer.disconnect();
		tree_mutation_observer.disconnect();
		window.removeEventListener("click", urlc_onclick);

		if (e.code !== 1000) {
			console.log("[URLC] Reconnecting...")
			urlc_make_socket();
		}
	}

	function urlc_socket_error(e) {
		console.error("[URLC] Socket error", e);
	}

	// Queue a link if necessary.
	function urlc_queue_element(element) {
		if (element.tagName !== "A" && element.tagName !== "AREA") {
			return;
		}

		let hrefattr = element.getAttribute("href");

		if (hrefattr === null || hrefattr === "#" || cache.get(element) === hrefattr) {
			return;
		}

		let href;

		// Getting `element.href` is expensive, so if `hrefattr` is absolute, just use that.
		if (/^https?:\/\//gi.test(hrefattr)) {
			href = hrefattr;
		} else {
			href = element.href;
		}

		let task = href;

		// Some websites have links that are best cleaned using details from places other than just their URL.
		if (host_category === "furaffinity" && element.matches(".user-contact-user-info a")) {
			task = JSON.stringify({
				url: "https://example.com/",
				context: {
					vars: {
						unmangle_mode: "contact_info",
						site: element.parentElement.querySelector("strong").innerHTML.toLowerCase(),
						text: element.innerText
					}
				}
			});
		} else if (host_category === "twitter" && href.startsWith("https://t.co/") && element.innerText.startsWith("http")) {
			task = element.childNodes[0].innerText + element.childNodes[1].textContent + (element.childNodes[2]?.innerText ?? "");
		} else if (host_category === "duckduckgo" && window.location.pathname == "/" && /[?&]q=/g.test(window.location.search) && element.matches("[data-testid^=result-]")) {
			task = element.closest("li").querySelector("a p").textContent.replaceAll(/\s\u203a\s/g, "/");
		}

		queue.push([new WeakRef(element), href]);

		if (tasks.length === 0) {
			send_timeout = setTimeout(urlc_send_tasks, 10);
		}
		tasks += task;
		tasks += "\n";
		if (tasks.length >= 2**18) {
			urlc_send_tasks();
		}
	}

	function urlc_send_tasks() {
		clearTimeout(send_timeout);
		send_timeout = null;
		socket.send(tasks);
		tasks = "";
	}

	// If the user clicks on an element in the queue, cancel the click and reclick the element once cleaned.
	function urlc_onclick(e) {
		for (let [queued_element, _] of queue) {
			if (queued_element.deref() == e.target) {
				e.preventDefault();
				reclick_once_clean = new WeakRef(e.target);
				return;
			}
		}
	}

	function urlc_href_mutation(mutations) {
		for (let mutation of mutations) {
			urlc_queue_element(mutation.target);
		}
	}

	function urlc_tree_mutation(mutations) {
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
}
