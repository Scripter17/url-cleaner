# URL Cleaner Site CLIent

A CLI client for URL Cleaner Site.

Supports HTTP, HTTPS, WS, and WSS.

## Why not Curl?

Previously Site didn't handle the `Expect: 100-continue` header correctly, making Curl wait 1 whole second before doing any of a job.

This has since been resolved, making `curl http://... -T tasks.txt` perfectly fine.

However, Site CLIent still has better ergonomics for sending the JobConfig.

## Why not just WebSocat?

Due to usually-good-but-here-annoying design decisions in Tungstenite, the WebSocket library Site uses, Site can only process one message at a time and cannot support continuing lines between messages.

WebSocat currently only supports sending data as either one line per message or one arbitrarily split chunk per message.

In one line per meesage mode, Site cannot process tasks in parallel and also has to deal with a ton more overhead.

In arbitrary chunk mode, tasks are often split across multiple messages, completely mangling both that task and the "nth line is the nth task" principle.

## Format

URL Cleaner Site Client uses [the standard format](../format.md).
