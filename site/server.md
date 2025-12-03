# URL Cleaner Site server management

By default, URL Cleaner Site will listen on IP `127.0.0.1` at port `9149`.

The API is described in [api.md](api.md).

Please note that URL Cleaner Site does not have any authentication/account system.

If you plan to open your instance of URL Cleaner Site to other computers (for example, by binding to IP `0.0.0.0` instead) you should at least use firewalls to stop unauthorized access.

For an overly annoying but theoretically sound account-style system, [mTLS](#mtls) can be used on top of [TLS](#tls) to make it impossible for unauthorized clients to connect to the server.

## TLS

TLS can be used with the `--key` and `--cert` arguments.

Additionally [mTLS](#mtls) is supported with the `--mtls-cert` argument.

Due to HTTP/HTTPS/TLS/whatever being bad and/or me being stupid, HTTP requests are not upgraded to HTTPS and instead seem to return garbage. Sorry!

To generate a public/private key pair, use the following OpenSSL commands with `YOUR_IP` changed to your instances local IP, which is usually `192.168.x.x`, `10.x.x.x`, or `172.16.x.x`.

You can add more `IP=1.2.3.4` and `DNS:example.com` to the list for public instances.

For AGPL compliance when using mTLS, a link to the source code of URL Cleaner is included in as a certificate extension. At the very least `curl -vk` shows the field.

```Bash
openssl genpkey -algorithm rsa -pkeyopt bits:4096 -quiet -out url-cleaner-site.key
openssl req -x509 -key url-cleaner-site.key -days 3650 -batch -subj "/CN=URL Cleaner Site" -addext "subjectAltName=DNS:localhost,IP:::1,IP:127.0.0.1,IP:YOUR_IP;sourceCode:ASN1:UTF8String:https:\/\/github.com\/Scripter17\/url-cleaner" -out url-cleaner-site.crt
```

Please note that TLS requires changing `window.URL_CLEANER_SITE = "ws://localhost:9149";` in the userscript to from `ws` to `wss`.

### Installing the certificate

#### Ios

1. Get the `url-cleaner-site.crt` file onto your iphone and open it such that you get a popup with "Profile Downloaded".

2. Open settings. Either tap the "Profile Downloaded" button at the top or, if it's not there, tap "General", scroll all the way down, then tap "VPN & Device Management"

3. Tap "URL Cleaner Site" under "Downloaded Profile".

4. Tap "Install" in the top right, authenticate, tap "Install" in the top right, then tap "Install" at the bottom, then tap "Done".

5. Go back one level (back into the "General" settings), scroll all the way up, tap "About", scroll all the way down, tap "Ceritifcate Trust Settings", find "URL Cleaner Site" under "Enable Full Trust For Root Certificate", then enable it.

#### Linux

```Bash
sudo cp url-cleaner-site.crt /local/usr/share/ca-certificates/
sudo update-ca-certificates
```

#### Firefox

For some reason, at least on my computer, Firefox ignores the above Linux setup. Simply opening `https://localhost:9149`, clicking "Advanced...", then clicking "Accept the Risk and Continue" seems to work fine.

Please note that due to a bug in Greasemonkey, setting `about:config`'s `privacy.firstparty.isolate` to `true` (as is default in forks like Mullvad Browser) breaks the userscript.

### mTLS

mTLS is an addition to TLS that lets servers require clients to have their own public/private key pair to prove their identity.

While URL Cleaner Site *should* support mTLS, I've yet to do any proper testing because nothing makes it easy to use.
