# URL Cleaner Site Userscript

## Installation

### Install userscript extension

[Greasemonkey] (for [Firefox][gm-f]),
[Tampermonkey] (for [chrome][tm-c], [edge][tm-e], [Firefox][tm-f], [safari][tm-s], and [opera][tm-o]),
[Userscripts] (for [safari][us-s]),
or any other userscript browser extension.

URL Cleaner Site Userscript is tested specifically on the latest release of Mullvad Browser with Greasemonkey, but all of the above extensions should work. Please tell me if they don't.

[Greasemonkey]: https://www.greasespot.net/
[gm-f]: https://addons.mozilla.org/en-US/firefox/addon/greasemonkey/

[Tampermonkey]: https://www.tampermonkey.net/
[tm-c]: https://chromewebstore.google.com/detail/dhdgffkkebhmkfjojejmpbldmpobfkfo
[tm-e]: https://microsoftedge.microsoft.com/addons/detail/iikmkjmpaadaobahmlepeloendndfphd
[tm-f]: https://addons.mozilla.org/en-US/firefox/addon/tampermonkey/
[tm-s]: https://apps.apple.com/app/tampermonkey/id6738342400
[tm-o]: https://addons.opera.com/en/extensions/details/tampermonkey-beta/

[Userscripts]: https://github.com/quoid/userscripts
[us-s]: https://apps.apple.com/us/app/userscripts/id1463298887

### Install URL Cleaner Site Userscript

Using your extension's method of adding userscripts, add [url-cleaner-site-userscript.json].

### Configure URL Cleaner Site Userscript

Near the start of the userscript is an object called `window.config` with various settings you should at least check if you care about.

### TLS/HTTPS

When using TLS, please see [here](../server.md#installing-the-certificate) for instructions on making your OS/browser accept your certificate.

Additionally, in the `instance` field of the `window.config`, change `ws://` to `wss://`.

## Known problems

Please note that due to a bug in Greasemonkey, setting `about:config`'s `privacy.firstparty.isolate` to `true` (as is default in forks like Mullvad Browser) breaks the userscript.

You can either set it to false in `about:config` or install a patched version of Greasemonkey, such as the one I submitted at <https://github.com/greasemonkey/greasemonkey/pull/3220>.

## Privacy

Please note that websites will be able to tell you're using URL Cleaner Site as well as which version of the Bundled Cleaner and what ParamsDiff you're using.

Additionally, URL Cleaner Site Userscript currently will clean every link on every webpage you visit.
Coupled with the Bundled Cleaner expanding all known redirects **unless the `no_network` flag is enabled, every website is able to send HTTP requests to bit.ly, t.co, etc. from your IP address**.

If this is a concern, there are several things you can do:

1. Make URL Cleaner Site use Tor/a VPN. `--proxy socks5://127.0.0.1:9150` should work fine, though I haven't actually tested it.

2. Enable the `no_network` flag in a named profile and set URL Cleaner Site Userscript to use that profile.

3. Enable the `no_network` flag in the base profile, which makes it apply to all profiles.

4. Enable the `no_network` flag in the ParamsDiff URL Cleaner Site Userscript sends to URL Cleaner Site. This shouldn't be *that* bad but you should almost always prefer using profiles.

Beyond this, there are two major privacy preserving features that most frontends disable by default but URL Cleaner Site Userscript specifically tells URL Cleaner Site to enable.

- Cache delays to make reading a cached value take about as long as the cached operation took.
  This prevents websites from checking if you've seen a URL before by how long it takes to clean.

- Unthreading to make long running operations like HTTP requests and cache reads effectively single threaded.
  This prevents websites from figuring out how many worker threads you use.

While neither of these protections are likely to stand up to state level enemies, they should at least allow the natural noise in how long jobs take to hide their respective details from normal observers.

Also if you're up against state level enemies maybe don't use URL Cleaner Site Userscript? Stick to URL Cleaner CLI.
