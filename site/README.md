# URL Cleaner Site

[![Crates.io Version](https://img.shields.io/crates/v/url-cleaner-site)](https://crates.io/crates/url-cleaner-site/)

A simple HTTP server to allow using URL Cleaner in web browser userscripts and other applications where SSH tunnels are infeasable.

Licensed under the Affero General Public License V3 or later (SPDX: AGPL-3.0-or-later)

https://www.gnu.org/licenses/agpl-3.0.html

## Usage

The main way to use URL Cleaner Site is with its userscript, [`url-cleaner-site-userscript.js`](url-cleaner-site-userscript.js).

Once you have URL Cleaner Site running on your computer, you can install the userscript using
[Greasemonkey](https://www.greasespot.net/) (for [Firefox](https://addons.mozilla.org/en-US/firefox/addon/greasemonkey/)),
[Tampermonkey](https://www.tampermonkey.net/) (for [chrome](https://chromewebstore.google.com/detail/dhdgffkkebhmkfjojejmpbldmpobfkfo), [edge](https://microsoftedge.microsoft.com/addons/detail/iikmkjmpaadaobahmlepeloendndfphd), [Firefox](https://addons.mozilla.org/en-US/firefox/addon/tampermonkey/), [safari](https://apps.apple.com/app/tampermonkey/id6738342400), and [opera](https://addons.opera.com/en/extensions/details/tampermonkey-beta/)),
[Userscripts](https://github.com/quoid/userscripts) (for [safari](https://apps.apple.com/us/app/userscripts/id1463298887)),
or any other userscript browser extension.

### Other devices

By default URL Cleaner Site will only accept traffic from the computer it's running on. If you want to use URL Cleaner site on another computer/phone:

1. Start URL Cleaner Site with `--bind 0.0.0.0` to make it accept requests from any computer on its network.

2. Before installing the userscript to your phone, first find the `// @connect localhost` and `instance: "http://localhost:9149"` lines near the top and change `localhost` to the local IP of the computer running URL Cleaner Site.
  Usually the IP looks like `192.168.x.x`, `10.x.x.x`, or `172.16.x.x`.

3. Optional but useful: Tell your router to always give the computer your instacne is running on the same local IP. On my router this feature is under "Basic", "LAN Setup", then "DHCP Reservation" and maps MAC addresses to IPs.
  Your router shouldn't ever randomly reassign you, especially if you're using a computer that's always online, but it does happen sometimes.

Once you've done that, your phone should be using URL Cleaner Site as long as it can see the server. If you want to use your instance globally you should use accounts and HTTPS.

### Accounts

If you want to use URL Cleaner Site everywhere, it should be safe to host a public instance using accounts.

An accounts file looks like

```Json
{
  "users": {
    "username1": "password1",
    "username2": "password2"
  },
  "allow_guests": true
}
```

and is specified using `--accounts accounts-file.json`.

When no accounts file is specified, it defaults to no accounts and allowing guests.

To adjust URL Cleaner Site Userscript to use accounts, find the `auth: null` near the top and replace it with `auth: {"username": "Your username", "password": "Your password"}`.

PLEASE note that there is currently no way to limit what an account can do. It's easy for a malicious user to ask your instance to clean a ton of redirect URLs in parallel, which can result in your instance's IP getting banned from that website.

Please also note that public instances should always use TLS (see below) to stop malicious networks from stealing your login details. By manually installing your self-signed certificate onto your devices using URL Cleaner Site, any router trying to use its own certificate will result in your device throwing errors and refusing to connect.

### HTTPS

HTTPS can be used with the `--key` and `--cert` arguments.

To generate a public/private key pair, use the following OpenSSL commands with `YOUR_IP` changed to your instances local IP, which is usually `192.168.x.x`, `10.x.x.x`, or `172.16.x.x`.

You can add more `IP=1.2.3.4` and `DNS:example.com` to the list for public instances.

```Bash
openssl genpkey -algorithm rsa -pkeyopt bits:4096 -quiet -out url-cleaner-site.key
openssl req -x509 -key url-cleaner-site.key -days 3650 -batch -subj "/CN=URL Cleaner Site" -addext "subjectAltName=DNS:localhost,IP:::1,IP:127.0.0.1,IP:YOUR_IP" -out url-cleaner-site.crt
```

Please note that HTTPS requires changing `window.URL_CLEANER_SITE = "http://localhost:9149";` in the userscript to from `http` to `https`.

#### Installing the certificate

##### Ios

1. Get the `url-cleaner-site.crt` file onto your iphone and open it such that you get a popup with "Profile Downloaded".

2. Open settings. Either tap the "Profile Downloaded" button at the top or, if it's not there, tap "General", scroll all thw way down, then tap "VPN & Device Management"

3. Tap "URL Cleaner Site" under "Downloaded Profile".

4. Tap "Install" in the top right, authenticate, tap "Install" in the top right, then tap "Install" at the bottom, then tap "Done".

5. Go back one level (back into the "General" settings), scroll all the way up, tap "About", scroll all the way down, tap "Ceritifcate Trust Settings", find "URL Cleaner Site" under "Enable Full Trust For Root Certificate", then enable it.

##### Linux

```Bash
sudo cp url-cleaner-site.crt /local/usr/share/ca-certificates/
sudo update-ca-certificates
```

##### Firefox

For some reason, at least on my computer, Firefox ignores the above Linux setup. Simply opening `https://localhost:9149`, clicking "Advanced...", then clicking "Accept the Risk and Continue" seems to work fine.

### mTLS

mTLS is an addition to HTTPS that lets servers require clients to have their own public/private key pair to prove their identity.

Unlike the account system, mTLS makes it impossible for unauthorized people to connect to the server at all.

While URL Cleaner Site *should* support mTLS, I've yet to do any proper testing because nothing makes it easy to use.

## Default cleaner

See [`../default_cleaner.md`](../default_cleaner.md) for details about the included default cleaner.

## Performance

Due to the overhead of using HTTP, the lack of streaming tasks and results, and optionally TLS, performance is significantly worse than the CLI.

On the same laptop used in URL Cleaner's example benchmarks and without TLS, hyperfine (using CURL) gave me the following benchmarks:

Last updated 2025-07-26.

```Json
{
  "https://x.com?a=2": {
    "0"    :  8.982,
    "1"    :  8.828,
    "10"   :  8.916,
    "100"  :  9.009,
    "1000" : 11.334,
    "10000": 31.007
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    :  8.779,
    "1"    :  8.788,
    "10"   :  8.799,
    "100"  :  9.109,
    "1000" : 12.332,
    "10000": 43.898
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    :  8.719,
    "1"    :  8.741,
    "10"   :  8.842,
    "100"  :  9.277,
    "1000" : 14.284,
    "10000": 58.170
  }
}
```

And with TLS:

```Json
{
  "https://x.com?a=2": {
    "0"    : 24.341,
    "1"    : 24.135,
    "10"   : 24.588,
    "100"  : 24.324,
    "1000" : 27.245,
    "10000": 50.623
  },
  "https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id": {
    "0"    : 24.526,
    "1"    : 24.622,
    "10"   : 24.739,
    "100"  : 24.540,
    "1000" : 28.543,
    "10000": 65.777
  },
  "https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8": {
    "0"    : 24.352,
    "1"    : 24.643,
    "10"   : 24.577,
    "100"  : 24.684,
    "1000" : 30.326,
    "10000": 82.949
  }
}
```

If you're using Firefox, you should know that Greasemonkey gives me much better performance of the userscript than Tampermonkey.

As for the performance of the userscript itself... I honestly can't say. Nothing strikes me as particularly bad in terms of either CPU or memory usage, but I haven't seriously used javascript in years.
It probably has a very slow memory leak that would be a problem when on a long-running webpage session having billions of elements, but that's very unlikely to ever happen outside testing.
