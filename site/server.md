# URL Cleaner Site server management

By default, URL Cleaner Site will listen on IP `127.0.0.1` at port `9149`.

The API is described in [api.md](api.md).

## Passwords

You can limit access to cleaning URLs by providing `--passwords` with a file containing one password per non-empty line.

If no password file is provided, users must not provide a password.

If a password file is provided, users must provided a password.

## TLS

1. Find your local IP address (usually `10.0.0.X`, `172.16.X.Y`, or `192.168.X.Y`)

2. Add `,IP:` followed by that IP address to the end of the second command below.

3. Run the following commands with that addition:

```Bash
openssl req -x509 -newkey rsa:2048 -keyout urlcs-ca.key -quiet -noenc -out urlcs-ca.crt -days 365 -subj "/CN=URL Cleaner Site CA"
openssl req -newkey rsa:2048 -keyout urlcs.key -quiet -noenc -out urlcs.csr -subj "/CN=URL Cleaner Site" -addext "subjectAltName=DNS:localhost,IP:::1,IP:127.0.0.1"
openssl x509 -req -in urlcs.csr -CA urlcs-ca.crt -CAkey urlcs-ca.key -out urlcs.crt -days 365 -copy_extensions copy
```

4. Install `urlcs-ca.crt` on the device(s) connecting to URL Cleaner Site. See below for explanations.

5. Change URL Cleaner Site to start with `--key urlcs.key --cert urlcs.crt`. Be sure to not use the `-ca` files.

6. Change clients from using `http://`/`ws://` to `https://`/`wss://`.

### Installing the certificate

#### Ios

1. Get the `urlcs-ca.crt` file onto your iphone and open it such that you get a popup with "Profile Downloaded".

2. Open settings. Either tap the "Profile Downloaded" button at the top or, if it's not there, tap "General", scroll all the way down, then tap "VPN & Device Management"

3. Tap "URL Cleaner Site" under "Downloaded Profile".

4. Tap "Install" in the top right, authenticate, tap "Install" in the top right, then tap "Install" at the bottom, then tap "Done".

5. Go back one level (back into the "General" settings), scroll all the way up, tap "About", scroll all the way down, tap "Ceritifcate Trust Settings", find "URL Cleaner Site" under "Enable Full Trust For Root Certificate", then enable it.

#### Linux

```Bash
sudo cp urlcs-ca.crt /usr/local/share/ca-certificates/
sudo update-ca-certificates
```

#### Firefox

For some reason, at least on my computer, Firefox ignores the above Linux setup. Simply opening `https://localhost:9149`, clicking "Advanced...", then clicking "Accept the Risk and Continue" seems to work fine.
