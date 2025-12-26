# Benchmarks

Measurements of how fast URL Cleaner's frontends are, as seen on the following hardware:

```
distro: Ubuntu 25.04 x86_64 
kernel: 6.14.0-061400-generic 
model: 20FAS39200 ThinkPad T460s 
cpu: Intel i5-6300U (4) @ 3.000GHz 
memory: 2386MiB / 11367MiB 
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
|Baseline|`6.4`|`6.5`|`6.5`|`6.6`|`8.3`|`22.4`|`184.5`|`1806.8`|
|UTPs|`6.4`|`6.4`|`6.5`|`6.7`|`9.3`|`30.7`|`268.4`|`2693.6`|
|Amazon|`6.3`|`6.4`|`6.6`|`6.7`|`9.1`|`34.2`|`336.5`|`3213.1`|
|Google|`6.3`|`6.4`|`6.5`|`6.7`|`9.5`|`36.3`|`354.1`|`3188.5`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`560,881`|`564,783`|`572,417`|`574,275`|`620,926`|`1,089,234`|`3,821,121`|`3,979,557`|
|UTPs|`564,983`|`564,575`|`573,063`|`577,775`|`692,358`|`1,779,802`|`4,586,685`|`4,835,521`|
|Amazon|`564,991`|`564,783`|`574,606`|`594,309`|`818,909`|`1,460,036`|`1,118,192`|`1,282,751`|
|Google|`564,783`|`566,277`|`574,227`|`595,448`|`897,391`|`973,727`|`917,413`|`1,375,941`|

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
|Baseline|`8.9`|`8.8`|`8.8`|`9.1`|`10.1`|`23.0`|`145.3`|`1365.9`|
|UTPs|`7.8`|`8.8`|`8.9`|`8.8`|`10.9`|`34.3`|`249.9`|`2442.6`|
|Amazon|`8.1`|`9.0`|`9.0`|`9.1`|`12.6`|`43.7`|`354.5`|`3371.6`|
|Google|`8.8`|`8.7`|`8.7`|`9.2`|`12.9`|`48.7`|`422.0`|`3960.8`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`783,633`|`764,589`|`765,728`|`774,107`|`859,424`|`1,603,240`|`7,710,786`|`78,870,328`|
|UTPs|`764,569`|`764,697`|`773,198`|`800,522`|`1,010,205`|`2,671,894`|`22,500,498`|`179,516,202`|
|Amazon|`783,633`|`764,799`|`774,634`|`821,641`|`1,376,609`|`6,081,075`|`40,668,612`|`328,581,812`|
|Google|`764,185`|`764,863`|`775,418`|`870,905`|`1,492,403`|`6,064,334`|`41,480,980`|`605,120,300`|

## Site WebSocket

Please note that a custom client is used to send multiple task configs per message, unlike WebSocat which can only do 1 per message.

This reduces the overhead of using WebSockets DRAMATICALLY. If your client isn't bundling tasks it'll likely be several times slower.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.0`|`3.9`|`4.2`|`4.5`|`6.8`|`27.0`|`240.4`|`2364.2`|
|UTPs|`3.2`|`4.1`|`4.2`|`4.4`|`7.8`|`38.7`|`345.9`|`3483.3`|
|Amazon|`3.1`|`3.9`|`3.8`|`4.6`|`8.7`|`46.2`|`408.0`|`4023.4`|
|Google|`3.7`|`4.4`|`4.4`|`4.3`|`9.4`|`52.6`|`466.6`|`4649.9`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`761,675`|`761,291`|`780,739`|`771,488`|`864,033`|`1,187,514`|`1,256,900`|`1,196,814`|
|UTPs|`761,675`|`761,675`|`766,541`|`796,114`|`1,043,034`|`1,074,613`|`1,134,400`|`1,094,226`|
|Amazon|`761,675`|`780,739`|`780,739`|`831,170`|`1,043,410`|`1,056,665`|`1,056,642`|`1,050,845`|
|Google|`761,291`|`761,675`|`770,192`|`848,564`|`1,042,958`|`1,051,754`|`1,045,125`|`1,052,821`|

