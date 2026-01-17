# URL Cleaner Site server management

By default, URL Cleaner Site will listen on IP `127.0.0.1` at port `9149`.

The API is described in [api.md](api.md).

## Passwords

You can limit access to cleaning URLs by providing `--passwords` with a JSON file containing an array of strings.

If no password file is provided, users must not provide a password.

If a password file is provided, users must provided a password.

## TLS

TLS can be used with the `--key` and `--cert` arguments.

To generate a public/private key pair, use the following OpenSSL commands.

```Bash
openssl genpkey -algorithm rsa -quiet -out urlcs.key
openssl req -x509 -key urlcs.key -days 3650 -batch -subj "/CN=URL Cleaner Site" -addext "subjectAltName=DNS:localhost,IP:::1,IP:127.0.0.1" -out urlcs.crt
```

To allow other computers on your network to trust the certificate, add `,IP:YOUR_LOCAL_IP` to the `subjectAltName`.

Please note that TLS requires changing `"instance": "ws://localhost:9149"` in the userscript to from `ws` to `wss`.

Unfortunately, URL Cleaner Site doesn't currently have any HTTPS upgrade mechanism.

### Installing the certificate

#### Ios

1. Get the `url-cleaner-site.cert` file onto your iphone and open it such that you get a popup with "Profile Downloaded".

2. Open settings. Either tap the "Profile Downloaded" button at the top or, if it's not there, tap "General", scroll all the way down, then tap "VPN & Device Management"

3. Tap "URL Cleaner Site" under "Downloaded Profile".

4. Tap "Install" in the top right, authenticate, tap "Install" in the top right, then tap "Install" at the bottom, then tap "Done".

5. Go back one level (back into the "General" settings), scroll all the way up, tap "About", scroll all the way down, tap "Ceritifcate Trust Settings", find "URL Cleaner Site" under "Enable Full Trust For Root Certificate", then enable it.

#### Linux

```Bash
sudo cp urlcs.crt /usr/local/share/ca-certificates/
sudo update-ca-certificates
```

#### Firefox

For some reason, at least on my computer, Firefox ignores the above Linux setup. Simply opening `https://localhost:9149`, clicking "Advanced...", then clicking "Accept the Risk and Continue" seems to work fine.
