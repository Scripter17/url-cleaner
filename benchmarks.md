# Benchmarks

Measurements of how fast URL Cleaner's frontends are, as seen on the following hardware:

```
distro: Ubuntu 25.04 x86_64 
kernel: 6.14.0-061400-generic 
model: 20FAS39200 ThinkPad T460s 
cpu: Intel i5-6300U (4) @ 3.000GHz 
memory: 2713MiB / 11367MiB 
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
|Baseline|`6.2`|`6.3`|`6.4`|`6.6`|`8.1`|`21.3`|`179.2`|`1643.3`|
|UTPs|`6.3`|`6.4`|`6.4`|`6.7`|`9.3`|`29.7`|`269.0`|`2600.7`|
|Amazon|`6.3`|`6.3`|`6.5`|`6.7`|`9.2`|`33.8`|`323.7`|`3247.1`|
|Google|`6.3`|`6.3`|`6.4`|`6.8`|`9.5`|`35.2`|`340.5`|`3326.5`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`561,402`|`561,402`|`568,828`|`570,097`|`621,406`|`1,087,131`|`3,657,860`|`4,088,884`|
|UTPs|`561,402`|`561,402`|`569,414`|`577,336`|`691,052`|`1,784,071`|`4,471,766`|`4,864,888`|
|Amazon|`561,402`|`561,402`|`569,979`|`590,720`|`809,509`|`1,335,261`|`2,277,024`|`1,372,891`|
|Google|`561,402`|`561,402`|`570,888`|`597,698`|`895,770`|`886,451`|`1,465,543`|`1,084,693`|

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
|Baseline|`8.5`|`8.0`|`8.2`|`8.7`|`9.4`|`22.6`|`147.2`|`1352.0`|
|UTPs|`7.4`|`8.3`|`8.8`|`8.7`|`11.1`|`34.4`|`255.2`|`2442.5`|
|Amazon|`7.9`|`8.5`|`8.7`|`8.9`|`12.2`|`43.4`|`359.6`|`3335.8`|
|Google|`8.4`|`8.5`|`8.6`|`9.0`|`12.9`|`50.1`|`417.8`|`3892.7`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`760,698`|`761,102`|`781,582`|`791,528`|`856,497`|`1,598,721`|`7,709,731`|`78,893,905`|
|UTPs|`761,082`|`780,658`|`768,866`|`799,699`|`1,006,486`|`2,700,882`|`22,496,455`|`178,823,519`|
|Amazon|`760,698`|`761,312`|`770,672`|`835,110`|`1,373,122`|`6,080,069`|`40,743,966`|`328,185,909`|
|Google|`780,146`|`780,824`|`771,619`|`867,418`|`1,488,873`|`6,107,480`|`41,477,503`|`601,857,285`|

## Site WebSocket

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.4`|`4.4`|`4.5`|`6.1`|`21.1`|`115.5`|`859.8`|`9710.2`|
|UTPs|`3.5`|`4.6`|`4.7`|`5.8`|`24.6`|`147.8`|`1431.8`|`13640.2`|
|Amazon|`3.6`|`4.5`|`4.1`|`6.5`|`24.4`|`166.5`|`1541.6`|`14642.8`|
|Google|`4.0`|`4.4`|`4.6`|`6.7`|`26.6`|`163.6`|`1627.1`|`15540.3`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`758,284`|`757,900`|`777,348`|`757,900`|`777,348`|`777,348`|`757,900`|`758,284`|
|UTPs|`757,900`|`777,348`|`777,348`|`758,284`|`757,900`|`757,900`|`777,348`|`777,348`|
|Amazon|`757,900`|`777,348`|`758,284`|`777,348`|`757,900`|`757,900`|`757,900`|`757,900`|
|Google|`757,900`|`757,900`|`777,348`|`757,900`|`757,900`|`758,284`|`757,900`|`757,900`|

