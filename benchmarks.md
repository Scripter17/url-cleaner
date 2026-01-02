# Benchmarks

Measurements of how fast URL Cleaner's frontends are, as seen on the following hardware:

```
distro: Ubuntu 25.04 x86_64 
kernel: 6.14.0-061400-generic 
model: 20FAS39200 ThinkPad T460s 
cpu: Intel i5-6300U (4) @ 3.000GHz 
memory: 2554MiB / 11367MiB 
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
|Baseline|`6.4`|`6.4`|`6.5`|`6.7`|`8.2`|`22.5`|`187.1`|`1662.5`|
|UTPs|`6.3`|`6.4`|`6.5`|`6.8`|`9.4`|`31.0`|`270.3`|`2577.4`|
|Amazon|`6.4`|`6.4`|`6.5`|`6.9`|`9.4`|`35.6`|`349.3`|`3253.0`|
|Google|`6.3`|`6.5`|`6.5`|`6.9`|`9.7`|`38.3`|`350.1`|`3485.2`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`564,606`|`564,406`|`572,040`|`572,505`|`620,583`|`1,088,051`|`3,749,932`|`4,374,768`|
|UTPs|`560,504`|`564,406`|`572,268`|`580,548`|`692,970`|`1,696,702`|`4,373,440`|`5,127,992`|
|Amazon|`564,606`|`564,406`|`570,698`|`593,932`|`834,725`|`1,230,732`|`1,905,773`|`2,659,519`|
|Google|`564,606`|`565,900`|`574,385`|`600,410`|`898,982`|`931,515`|`1,706,625`|`1,473,101`|

## Site HTTP

Measured with `curl http://... -H Expect: --data-binary @stdin.txt`.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`8.7`|`8.8`|`8.5`|`8.9`|`10.1`|`32.8`|`163.0`|`1402.5`|
|UTPs|`7.9`|`8.8`|`8.9`|`8.5`|`10.9`|`39.7`|`234.5`|`2277.7`|
|Amazon|`7.8`|`8.6`|`8.8`|`9.0`|`11.5`|`70.3`|`286.9`|`2779.2`|
|Google|`8.1`|`8.8`|`8.8`|`8.5`|`11.8`|`66.6`|`321.5`|`3121.7`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`729,386`|`728,962`|`710,194`|`728,962`|`789,357`|`1,161,278`|`2,080,166`|`2,111,920`|
|UTPs|`710,618`|`729,818`|`710,194`|`736,178`|`970,239`|`2,024,646`|`2,158,413`|`2,136,479`|
|Amazon|`728,466`|`711,136`|`710,703`|`778,014`|`1,139,179`|`2,284,494`|`2,268,828`|`2,192,646`|
|Google|`710,554`|`728,962`|`728,962`|`778,958`|`1,510,334`|`2,320,863`|`2,197,431`|`2,276,518`|

## Site HTTPS

Measured with `curl https://... -H Expect: --data-binary @stdin.txt`.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`72.7`|`73.3`|`73.2`|`74.0`|`71.2`|`87.7`|`221.5`|`1543.4`|
|UTPs|`72.2`|`72.9`|`73.6`|`73.1`|`77.6`|`92.0`|`331.2`|`2514.3`|
|Amazon|`73.4`|`73.1`|`74.1`|`72.4`|`78.0`|`99.5`|`385.9`|`3188.8`|
|Google|`72.0`|`73.4`|`72.6`|`72.6`|`77.1`|`108.2`|`431.6`|`3583.1`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`763,733`|`765,665`|`765,724`|`767,630`|`822,022`|`1,068,228`|`2,050,835`|`2,052,437`|
|UTPs|`763,621`|`765,160`|`766,274`|`786,565`|`940,438`|`1,747,056`|`2,150,734`|`2,124,042`|
|Amazon|`763,581`|`764,626`|`767,725`|`798,580`|`1,007,279`|`2,180,548`|`2,238,426`|`2,249,028`|
|Google|`766,029`|`764,131`|`768,136`|`804,748`|`1,117,405`|`2,310,292`|`2,318,420`|`2,303,960`|

## Site WebSocket

Please note that [URL Cleaner Site WebSocket Client](site-ws-client) is used to send multiple task configs per message.

This dramatically reduces the overhead of using WebSockets, to the point of making large jobs several times faster.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`3.8`|`4.0`|`4.0`|`4.2`|`5.8`|`19.9`|`158.0`|`1511.5`|
|UTPs|`2.9`|`3.8`|`3.9`|`4.4`|`7.3`|`34.0`|`282.8`|`2830.5`|
|Amazon|`2.8`|`4.0`|`4.1`|`4.2`|`13.4`|`46.4`|`387.9`|`3710.9`|
|Google|`3.3`|`4.1`|`3.7`|`4.6`|`20.2`|`51.6`|`433.4`|`4357.2`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`833,242`|`833,242`|`833,242`|`833,458`|`891,517`|`1,260,048`|`1,260,015`|`1,260,145`|
|UTPs|`833,242`|`833,242`|`833,242`|`834,497`|`1,103,118`|`1,132,144`|`1,127,459`|`1,395,368`|
|Amazon|`833,181`|`833,181`|`830,810`|`866,417`|`1,107,470`|`1,122,479`|`1,112,310`|`1,374,457`|
|Google|`832,858`|`830,810`|`833,242`|`879,217`|`1,104,128`|`1,104,628`|`1,101,255`|`1,370,900`|

## Site WebSocket Secure

Please note that [URL Cleaner Site WebSocket Client](site-ws-client) is used to send multiple task configs per message.

This dramatically reduces the overhead of using WebSockets, to the point of making large jobs several times faster.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`31.6`|`31.9`|`31.6`|`32.4`|`34.3`|`49.8`|`187.8`|`1561.0`|
|UTPs|`30.7`|`31.7`|`32.0`|`32.4`|`52.5`|`62.7`|`320.5`|`2889.3`|
|Amazon|`31.1`|`32.0`|`31.8`|`32.4`|`39.6`|`74.3`|`433.1`|`3994.3`|
|Google|`31.0`|`32.0`|`32.1`|`32.7`|`39.8`|`80.8`|`488.4`|`4612.4`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`880,152`|`879,992`|`880,152`|`880,152`|`965,271`|`1,201,323`|`1,201,299`|`1,201,420`|
|UTPs|`879,992`|`879,992`|`880,152`|`894,604`|`1,044,446`|`1,073,729`|`1,073,297`|`1,087,883`|
|Amazon|`880,152`|`880,152`|`880,152`|`939,486`|`1,049,684`|`1,047,670`|`1,049,684`|`1,057,748`|
|Google|`880,152`|`880,152`|`880,152`|`952,626`|`1,045,199`|`1,044,998`|`1,044,652`|`1,043,707`|

