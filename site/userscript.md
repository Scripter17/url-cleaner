# URL Cleaner Site Userscript

## Installation

### Install userscript extension

[Greasemonkey] (for [Firefox][gm-f]),
[Tampermonkey] (for [chrome][tm-c], [edge][tm-e], [Firefox][tm-f], [safari][tm-s], and [opera][tm-o]),
[Userscripts] (for [safari][us-s]),
or any other userscript browser extension.

URL Cleaner Site Userscript is tested specifically on the latest release of Mullvad Browser with [Greasemonkey] and ios safari with [Userscripts], but all of the above extensions should work. Please tell me if they don't.

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

### Get URL Cleaner Site Userscript

To get URL Cleaner Site Userscript, you can either:

- See [src/userscript.js](src/userscript.js).

  - With this, you need to fill out the instance info manually (if applicable).

- GET the `/userscript` endpoint of some URL Cleaner Site instance.

  - Can be done with URL Cleaner Site CLIent using the `get` subcommand.

- Use URL Cleaner Site's `userscript` subcommand.

Afterwards, add the userscript you your userscript manager.

### TLS/HTTPS

When using TLS, please see [here](server.md#installing-the-certificate) for instructions on making your OS/browser accept your certificate.

## Known problems

### Ios

Because apple doesn't know how to do anything, [Userscripts] is unable to connect to HTTP instances of URL Cleaner Site on HTTPS websites.

The solution is to make URL Cleaner Site use HTTPS. Sorry.

## Privacy

Please note that websites will be able to tell you're using URL Cleaner Site as well as which version of the Bundled Cleaner and what ParamsDiff you're using.

Additionally, URL Cleaner Site Userscript currently cleans every link on every webpage you visit.

Coupled with the Bundled Cleaner expanding all known redirects, **unless the `no_network` flag is enabled, every website is able to send HTTP requests to bit.ly, t.co, etc. from your IP address**.

If this is a concern, there are several things you can do:

- Make URL Cleaner Site use a proxy by setting the `ALL_PROXY` environment variable.

- When using specifically the Bundled Cleaner or any custom Cleaner with an equivalent feature:

  - Enable the `no_network` flag in the base profile, which makes it apply to all profiles.

  - Enable the `no_network` flag in a named profile and set URL Cleaner Site Userscript to use that profile.

  - Enable the `no_network` flag in the ParamsDiff URL Cleaner Site Userscript sends to URL Cleaner Site. Please consider this a last resort because it adds a ponderable amount of work for Site to do at the start of each connection.

Beyond this, there are two major privacy preserving features that most frontends disable by default but URL Cleaner Site Userscript specifically tells URL Cleaner Site to enable.

- Cache delays to make reading a cached value take about as long as the cached operation took.
  This prevents websites from checking if you've seen a URL before by how long it takes to clean.

- Unthreading to make long running operations like HTTP requests and cache reads effectively single threaded.
  This prevents websites from figuring out how many worker threads you use.

While neither of these protections are likely to stand up to state level enemies, they should at least allow the natural noise in how long jobs take to hide their respective details from normal observers.

Also if you're up against state level enemies maybe don't use URL Cleaner Site Userscript? Stick to URL Cleaner CLI.
