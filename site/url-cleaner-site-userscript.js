// ==UserScript==
// @name         URL Cleaner Site Userscript
// @description  The userscript that comes with URL Cleaner Site.
// @author       Scripter17@Github.com
// @namespace    https://github.com/Scripter17/url-cleaner
// @copyright    AGPL-3.0-or-later
// @version      0.12.0-beta
// @match        https://*/*
// @match        http://*/*
// @grant        GM.xmlHttpRequest
// @connect      localhost
// ==/UserScript==

window.config = {
	instance   : "http://localhost:9149", // The origin (protocol://host:port) of your URL Cleaner Site instance. When changing, please also update the "// @connect" line above.
	auth       : null, // Either null for guest mode or `{"username": "...", "password": "..."}` for a user.
	profile    : null, // Either null for the default profile or a string for a named profile.
	params_diff: null, // The ParamsDiff to apply on top of the chosen profile. Should only be used as a last resort.
	send_host  : true, // If true, tells URL Cleaner Site the host of the webpage you're on so it can clean stuff the website does.
	cache_delay: true, // Artifically delay cache reads to take about as long as the initial run to defend against cache detection.
	unthread   : true, // Makes requests, cache reads, etc. effectively single threaded to hide thread count.
	debug: {
		log: {
			new_clean_payload: false,
			api_request_info : false,
			api_request_error: true,
			api_response_info: false,
			other_timing_info: false,
			href_mutations   : false,
			server_info      : false
		}
	}
};

window.SERVER_INFO = null;

window.cleaned_elements = new WeakMap(); // A map from elements to the last value this userscript set its href to. Used to check if a mutation is relevant.
window.too_big_elements = new WeakSet(); // Set of elements whose hrefs were bigger than URL Cleaner Site's max size.
window.errored_elements = new WeakSet(); // Set of elements whose hrefs returned an error.
window.total_elements_cleaned = 0;
window.total_time_cleaning = 0;

async function main_loop() {
	var elements = [...document.links]
		.filter(e => !e.getAttribute("href").startsWith("#") && // Websites often use `href="#"` to make buttons work on slop browsers like ios safari.
			!window.cleaned_elements.has(e) && !window.too_big_elements.has(e) && !window.errored_elements.has(e) // Make sure we didn't already handle it.
		);
	await clean_elements(elements);
	setTimeout(main_loop, 100); // Is this a good interval? No idea. Is an interval even the right approach? Also no idea.
}

// The `clean_payload`s parameter is used to make breaking big jobs into parts faster. I think.
async function clean_elements(elements, clean_payload) {

	if (elements.length == 0) {return;}

	clean_payload ??= await elements_to_clean_payload(elements);

	// If the `clean_payload` is too big, break it into parts.
	let data = JSON.stringify(clean_payload);
	if (data.length > window.SERVER_INFO.max_json_size) {
		if (elements.length == 1) {
			// If, somehow, there's a URL that's over the server's size limit, this stops it from getting stuck in an infinite loop.
			console.error(`[URLC] URL Cleaner element too big error: ${elements[0]}`);
			window.too_big_elements.add(elements[0]);
			return;
		} else {
			// Cut the list in half and do them separately.
			await clean_elements(elements.slice(0, elements.length/2), {...clean_payload, tasks: clean_payload.tasks.slice(0, clean_payload.tasks.length/2)});
			elements = elements.slice(elements.length/2);
			clean_payload.tasks = clean_payload.tasks.slice(clean_payload.tasks.length/2);
			data = JSON.stringify(clean_payload);
		}
	}

	let start_time = new Date();
	let id = Math.floor(Math.random()*1e8); // Random to avoid iframes from being confusing.
	let id_pad = " ".repeat(8-id.toString().length)
	let last_time = start_time;
	let now;
	let done;
	let doneawaiter = new Promise(resolve => {done = resolve;});
	if (window.config.debug.log.new_clean_payload) {console.log("[URLC]"+id_pad, id, elements.length, "elements in", data.length, "bytes (", clean_payload, ")");}
	// This returns `undefined` in GreaseMonkey, so the weird "await for callback" pattern is required.
	await GM.xmlHttpRequest({
		url: `${window.config.instance}/clean`,
		method: "POST",
		data: data,
		timeout: 10000,
		onabort           : (e) => {if (window.config.debug.log.api_request_error) {now = new Date(); console.error("[URLC]"+id_pad, id, "abort            took", now-last_time, "ms (", e, ")"); last_time = now;} done();},
		onerror           : (e) => {if (window.config.debug.log.api_request_error) {now = new Date(); console.error("[URLC]"+id_pad, id, "error            took", now-last_time, "ms (", e, ")"); last_time = now;} done();},
		onloadstart       : (e) => {if (window.config.debug.log.api_request_info ) {now = new Date(); console.log  ("[URLC]"+id_pad, id, "loadstart        took", now-last_time, "ms (", e, ")"); last_time = now;}},
		onloadprogress    : (e) => {if (window.config.debug.log.api_request_info ) {now = new Date(); console.log  ("[URLC]"+id_pad, id, "loadprogress     took", now-last_time, "ms (", e, ")"); last_time = now;}},
		onreadystatechange: (e) => {if (window.config.debug.log.api_request_info ) {now = new Date(); console.log  ("[URLC]"+id_pad, id, "readystatechange took", now-last_time, "ms (", e, ")"); last_time = now;}},
		ontimeout         : (e) => {if (window.config.debug.log.api_request_error) {now = new Date(); console.error("[URLC]"+id_pad, id, "timeout          took", now-last_time, "ms (", e, ")"); last_time = now;} done();},
		onload: function(response) {
			if (window.config.debug.log.api_response_info) {now = new Date(); console.log("[URLC]"+id_pad, id, "load             took", now-last_time, "ms (", response, ")"); last_time = now;}
			let result = JSON.parse(response.responseText);
			if (result.Err == null) {
				result.Ok.urls.forEach(function (cleaning_result, index) {
					if (cleaning_result.Err == null) {
						if (elements[index].href != cleaning_result.Ok) {
							elements[index].setAttribute("href", cleaning_result.Ok);
						}
						window.cleaned_elements.set(elements[index], cleaning_result.Ok);
					} else {
						console.error("[URLC]"+id_pad, id, "DoTaskError:", cleaning_result.Err, "Element indesx:", index, "Element:", elements[index], "Task:", clean_payload.tasks[index]);
						window.errored_elements.add(elements[index])
					}
				});
			} else {
				console.error("[URLC]"+id_pad, id, "job config error", result);
			}
			now = new Date();
			window.total_time_cleaning += now-start_time;
			window.total_elements_cleaned += elements.length;
			if (window.config.debug.log.other_timing_info) {console.log("[URLC]"+id_pad, id, "writing          took", now-last_time , "ms");}
			if (window.config.debug.log.other_timing_info) {console.log("[URLC]"+id_pad, id, "all              took", now-start_time, "ms");}
			if (window.config.debug.log.other_timing_info) {console.log("[URLC]", "Total cleaning took", window.total_time_cleaning, "ms for", window.total_elements_cleaned, "elements");}
			done();
		}
	});
	await doneawaiter;
}

async function elements_to_clean_payload(elements) {
	let ret = {
		tasks: elements.map(x => element_to_task_config(x)),
	};
	let job_context = await get_job_context();
	if (job_context                       ) {ret.context     = job_context;}
	if (window.config.auth                ) {ret.auth        = window.config.auth;}
	if (window.config.profile     !== null) {ret.profile     = window.config.profile;}
	if (window.config.params_diff         ) {ret.params_diff = window.config.params_diff;}
	if (window.config.cache_delay !== null) {ret.cache_delay = window.config.cache_delay;}
	if (window.config.unthread    !== null) {ret.unthread    = window.config.unthread;}
	return ret;
}

function element_to_task_config(element) {
	if (/(^|\.)x\.com$/.test(window.location.hostname) && element.href.startsWith("https://t.co/") && element.innerText.startsWith("http")) {
		// On twitter, links in tweets/bios/whatever show the entire URL when you hover over them for a moemnt.
		// This lets us skip the HTTP request to t.co for the vast majority of links on twitter.
		return element.childNodes[0].innerText + element.childNodes[1].textContent + (element.childNodes[2]?.innerText ?? "");
	} else if (/(^|\.)allmylinks\.com$/.test(window.location.hostname) && element.pathname=="/link/out" && element.title.startsWith("http")) {
		// Same shortcut thing as the twitter stuff above.
		return element.title;
	} else if (/(^|\.)furaffinity\.net$/.test(window.location.hostname) && element.matches(".user-contact-user-info a")) {
		// Allows unmangling contact info links.

		if (element.href == "javascript:void(0);") {
			return "https://" + element.innerText;
		} else {
			// Some contact info fields let invalid inputs result in invalid URLs, which URL Cleaner can't accept.
			// Since the unmangling for this doesn't touch the actual URL, just replace it with a dummy.
			return {
				url: "https://example.com/url_cleaner_dummy",
				context: {
					vars: {
						contact_info_site_name: element.parentElement.querySelector("strong").innerHTML,
						link_text: element.innerText
					}
				}
			};
		}
	} else if (/(^|\.)bsky\.app$/.test(window.location.hostname) && element.getAttribute("href").startsWith("/profile/did:plc:") && element.innerText.startsWith("@")) {
		// Allows replacing `/profile/did:plc:` URLs with the `/profile/example.com`, as it should be.
		return eleement.href.replace(/did:plc:[^/]+/g, element.innerText.replace("@", ""))
	} else if (/(^|\.)saucenao\.com$/.test(window.location.hostname) && /^https:\/\/(www\.)?(x|twitter)\.com\//.test(element.href)) {
		// Fixes twitter URLs returned by SauceNAO.
		return element.href.replace(/i\/web|i\/user\/\d+/g, element.parentElement.querySelector("[href*='/i/user/']").innerHTML.replace("@", ""));
	} else {
		return element.href;
	}
}

// Because the webpage's URL can change without reloading the script, this needs to be calculated per job config.
async function get_job_context() {
	let ret = {};

	if (window.config.send_host) {
		ret.source_host = window.location.hostname;
	}

	return ret;
}

(async () => {
	console.log(`[URLC] URL Cleaner Site Userscript ${GM.info.script.version} loaded.
Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)
https://www.gnu.org/licenses/agpl-3.0.html
${GM.info.script.namespace}`);

	// For reasons I don't understand, awaiting `GM.xmlHttpRequest` doesn't seem to, uh, await it.
	// It might be me being stupid.
	let done;
	let doneawaiter = new Promise(resolve => {done = resolve;});
	await GM.xmlHttpRequest({
		url: `${window.config.instance}/info`,
		method: "GET",
		onerror: (e) => {console.error("[URLC] Failed to get server info:", e);},
		onload: function(response) {
			window.SERVER_INFO = JSON.parse(response.responseText);
			if (window.config.debug.log.server_info) {
				console.log("[URLC] Server info:", window.SERVER_INFO);
			}
			done();
		}
	});
	await doneawaiter;

	new MutationObserver(function(mutations) {
		if (window.config.debug.log.href_mutations) {console.log("[URLC]", "Href mutations observed (", mutations, ")");}
		mutations.forEach(function(mutation) {
			if (window.cleaned_elements.get(mutation.target) != mutation.target.href) {
				window.cleaned_elements.delete(mutation.target);
				window.too_big_elements.delete(mutation.target);
				window.errored_elements.delete(mutation.target);
				if (mutation.target.matches(":hover, :active, :focus, :focus-visible, :focus-within")) {
					mutation.target.addEventListener("click", async function(e) {
						if (window.cleaned_elements.has(e.target) || window.too_big_elements.has(e.target) || window.errored_elements.has(e.target)) {
							return;
						}
						e.preventDefault();
						try {
							await clean_elements([e.target]);
						} catch (err) {
							console.error("[URLC] Error with handling an uncleaning clickjack:", e, err);
							e.target.click();
							throw err;
						}
						e.target.click();
					}, {capture: true, once: true});
				}
			}
		});
	}).observe(document.documentElement, {
		attributes: true,
		attributeFilter: ["href"],
		subtree: true
	});

	await main_loop();
})();
