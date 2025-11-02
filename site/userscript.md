# URL Cleaner Site Userscript

## Install userscript extension

[Greasemonkey] (for [Firefox][gm-f]),
[Tampermonkey] (for [chrome][tm-c], [edge][tm-e], [Firefox][tm-f], [safari][tm-s], and [opera][tm-o]),
[Userscripts] (for [safari][us-s]),
or any other userscript browser extension.

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

## Install URL Cleaner Site Userscript

Using your extension's method of adding userscripts, add [url-cleaner-site-userscript.json].

### Configure URL Cleaner Site Userscript

Near the start of the userscript is an object called `window.config`.

If you are running URL Cleaner Site on another computer or have changed any settings in the server, please give it a quick look to make sure any changes are properly used.

## Known problems

Greasemonkey has a bug where HTTP requests don't work when `privacy.firstparty.isolate` is `true`.

You can either set it to false or install a patched version, such as the one I submitted at <https://github.com/greasemonkey/greasemonkey/pull/3220>.
