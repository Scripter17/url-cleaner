# Benchmarks

Measurements of how fast URL Cleaner's frontends are, as seen on the following hardware:

```
distro: Ubuntu 25.04 x86_64 
kernel: 6.14.0-061400-generic 
model: 20FAS39200 ThinkPad T460s 
cpu: Intel i5-6300U (4) @ 3.000GHz 
memory: 2252MiB / 11367MiB 
```

## Tasks

The tasks that are benchmarked.

|Name|Description|Task|
|:--|:--|:--|
|Baseline|An already clean URL|`https://example.com`|
|UTPs|Baseline with some universal tracking parameters|`https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id`|
|Amazon|An amazon product listing|`https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8`|
|Google|A google search result|`https://www.google.com/search?q=url+cleaner&sca_esv=de6549fe37924183&ei=eRAYabb6O7Gb4-EP79Xe6A8&ved=0ahUKEwj2mqWLt_OQAxWxzTgGHe-qF_0Q4dUDCBE&oq=url+cleaner&gs_lp=Egxnd3Mtd2l6LXNlcnAiC3VybCBjbGVhbmVySABQAFgAcAB4AZABAJgBAKABAKoBALgBDMgBAJgCAKACAJgDAJIHAKAHALIHALgHAMIHAMgHAA&sclient=gws-wiz-serp`|

## Cli

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`6.4`|`6.4`|`6.5`|`6.6`|`8.1`|`21.6`|`179.9`|`1654.3`|
|UTPs|`6.3`|`6.4`|`6.5`|`6.7`|`9.3`|`30.9`|`293.4`|`2614.6`|
|Amazon|`6.3`|`6.4`|`6.5`|`6.7`|`9.4`|`36.7`|`328.1`|`3154.5`|
|Google|`6.4`|`6.4`|`6.5`|`6.8`|`9.7`|`38.4`|`357.7`|`3489.9`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`564,870`|`564,878`|`566,071`|`571,069`|`624,882`|`1,088,681`|`4,878,972`|`4,790,712`|
|UTPs|`560,768`|`564,670`|`572,177`|`581,458`|`692,567`|`1,788,027`|`4,626,260`|`4,926,834`|
|Amazon|`564,870`|`564,878`|`570,962`|`591,716`|`834,636`|`1,073,832`|`2,082,115`|`2,053,436`|
|Google|`564,670`|`564,878`|`573,766`|`593,801`|`899,780`|`1,868,766`|`1,585,086`|`2,618,934`|

## Site HTTP

The max payload was increased from 25MiB to 1GiB.

Below is a table of how many of each task can actually fit in the default and increased limits.

|Name|Bytes|Lines in 25MiB|Lines in 1GiB|
|:--|--:|--:|--:|
|Baseline|`19`|`1,310,720`|`53,687,091`|
|UTPs|`89`|`291,271`|`11,930,464`|
|Amazon|`229`|`113,975`|`4,668,442`|
|Google|`293`|`89,164`|`3,652,183`|

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`8.8`|`8.6`|`9.0`|`8.8`|`10.3`|`22.5`|`145.7`|`1368.7`|
|UTPs|`7.8`|`8.9`|`8.9`|`8.9`|`11.3`|`34.1`|`253.5`|`2457.8`|
|Amazon|`8.1`|`8.8`|`8.7`|`8.9`|`12.3`|`44.0`|`368.8`|`3348.8`|
|Google|`8.7`|`8.7`|`8.7`|`9.1`|`12.7`|`49.8`|`416.7`|`3932.5`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`764,544`|`764,948`|`765,128`|`776,878`|`859,484`|`1,603,851`|`7,717,579`|`78,905,071`|
|UTPs|`764,544`|`784,504`|`768,260`|`800,468`|`1,010,127`|`2,674,523`|`22,501,093`|`176,447,773`|
|Amazon|`783,992`|`765,851`|`786,676`|`819,613`|`1,376,968`|`6,083,926`|`40,669,112`|`329,617,713`|
|Google|`783,992`|`765,222`|`795,254`|`871,264`|`1,492,554`|`6,090,700`|`41,482,381`|`606,383,815`|

## Site WebSocket

### Speed

Measured in milliseconds.

It seems this being so much slower than CLI is due to WebSocat sending each task line as its own message.

Using `--binary` gives very similar timings but is inadmissable here because it doesn't always chunk on line separators.

When making a client for Site WebSocket, you should try to send multiple task lines per message to reduce the overhead.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.5`|`4.1`|`4.6`|`5.9`|`21.1`|`120.9`|`875.6`|`10182.6`|
|UTPs|`4.1`|`4.6`|`4.7`|`6.3`|`24.4`|`152.0`|`1453.3`|`14167.4`|
|Amazon|`3.8`|`4.6`|`4.2`|`6.3`|`25.2`|`161.7`|`1492.7`|`14642.2`|
|Google|`3.5`|`4.8`|`4.5`|`6.6`|`26.0`|`179.4`|`1589.4`|`16131.6`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`781,194`|`781,194`|`761,746`|`761,746`|`761,746`|`781,194`|`761,746`|`754,368`|
|UTPs|`761,746`|`761,746`|`781,194`|`761,746`|`781,194`|`762,130`|`761,746`|`761,746`|
|Amazon|`762,130`|`761,746`|`761,746`|`781,194`|`761,746`|`781,194`|`761,746`|`781,194`|
|Google|`762,130`|`761,746`|`761,746`|`762,130`|`781,194`|`781,194`|`781,194`|`761,746`|

