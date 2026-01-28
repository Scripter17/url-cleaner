# URL Cleaner Site Client

An experimental CLI client for URL Cleaner Site.

Supports HTTP, HTTPS, WS, and WSS.

## Known issues

For reasons beyond my understanding, WebSocket Secure (wss://) connections will sometimes get all results then panic about the connection closing wrongly.

You should stick to HTTPS for now.

## Why not just Curl?

For large payloads, Curl expects an HTTP 100 early hint response thingy that Hyper, which Site uses internally, seems to not handle correctly.

This means that, unless you pass `-H Expect:` to Curl, Curl will wait 1 whole second before sending the job, which is often longer than te job itself.

If you must use Curl, my reccomendation is to do `print_job | curl http://.../clean -T - -H Expect:`.

## Why not just WebSocat?

While WebSocat is great for general WebSocket usage, it has no way of bundling multiple full lines into each message.

For complex reasons that are mostly beyond my control, this limitation makes WebSocat usually dozens of times slower than Site Client.

If you must use WebSocat, make sure it's using text mode so it doesn't split lines across multiple messages.

## Format

URL Cleaner Site Client uses [the standard format](../format.md).
