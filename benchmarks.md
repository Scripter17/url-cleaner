# Benchmarks

Measurements of how fast URL Cleaner's frontends are, as seen on the following hardware:

```
distro: Ubuntu 25.04 x86_64 
kernel: 6.14.0-061400-generic 
model: 20FAS39200 ThinkPad T460s 
cpu: Intel i5-6300U (4) @ 3.000GHz 
memory: 2803MiB / 11367MiB 
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
|Baseline|`6.2`|`6.4`|`6.4`|`6.6`|`8.2`|`21.6`|`182.9`|`1763.0`|
|UTPs|`6.3`|`6.4`|`6.4`|`6.7`|`9.3`|`30.2`|`302.0`|`2628.8`|
|Amazon|`6.3`|`6.3`|`6.4`|`6.8`|`9.2`|`33.9`|`315.7`|`3430.0`|
|Google|`6.3`|`6.3`|`6.4`|`6.8`|`9.5`|`36.2`|`329.3`|`3288.5`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`564,614`|`564,406`|`570,516`|`573,273`|`624,690`|`1,090,591`|`3,323,212`|`3,956,196`|
|UTPs|`564,614`|`564,614`|`573,411`|`579,970`|`676,578`|`1,705,390`|`4,547,418`|`5,014,546`|
|Amazon|`564,606`|`564,406`|`574,228`|`593,932`|`768,947`|`1,184,342`|`1,229,634`|`1,399,211`|
|Google|`564,198`|`564,614`|`573,179`|`600,376`|`812,306`|`1,185,593`|`1,127,418`|`1,222,840`|

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
|Baseline|`8.7`|`8.6`|`8.9`|`8.8`|`10.0`|`22.9`|`146.1`|`1377.4`|
|UTPs|`7.6`|`8.6`|`8.8`|`9.0`|`11.2`|`35.1`|`249.9`|`2443.6`|
|Amazon|`8.1`|`8.9`|`8.8`|`9.1`|`12.3`|`42.7`|`353.5`|`3281.7`|
|Google|`8.7`|`8.5`|`8.8`|`9.1`|`12.9`|`48.5`|`433.3`|`3923.0`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`764,202`|`764,222`|`767,218`|`772,942`|`858,957`|`1,603,065`|`7,712,921`|`78,904,461`|
|UTPs|`764,202`|`764,330`|`774,205`|`799,782`|`1,009,662`|`2,704,002`|`22,500,055`|`179,515,975`|
|Amazon|`763,818`|`783,880`|`773,792`|`821,274`|`1,376,242`|`6,080,358`|`41,433,089`|`334,293,874`|
|Google|`764,202`|`764,880`|`775,355`|`870,922`|`1,635,762`|`6,081,753`|`41,481,354`|`605,747,679`|

## Site WebSocket

Please note that [URL Cleaner Site WebSocket Client](site-ws-client) is used to send multiple task configs per message.

This dramatically reduces the overhead of using WebSockets, to the point of making large jobs several times faster.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`3.8`|`4.0`|`4.0`|`4.3`|`5.9`|`19.9`|`163.3`|`1563.7`|
|UTPs|`3.2`|`4.0`|`4.0`|`4.4`|`7.3`|`32.8`|`296.2`|`2829.6`|
|Amazon|`2.9`|`3.9`|`3.9`|`4.5`|`8.4`|`41.8`|`350.8`|`3452.4`|
|Google|`3.1`|`4.2`|`4.2`|`4.5`|`9.1`|`48.6`|`418.3`|`4092.1`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`760,924`|`761,308`|`780,372`|`780,372`|`861,651`|`1,196,494`|`1,197,010`|`1,193,438`|
|UTPs|`760,924`|`760,924`|`763,695`|`792,129`|`1,039,639`|`1,064,118`|`1,069,169`|`1,064,154`|
|Amazon|`761,308`|`760,924`|`765,882`|`835,644`|`1,041,224`|`1,044,719`|`1,042,954`|`1,048,919`|
|Google|`761,308`|`761,308`|`764,885`|`847,474`|`1,042,666`|`1,040,644`|`1,041,351`|`1,037,746`|

