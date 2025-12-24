# Benchmarks

Measurements of how fast URL Cleaner's frontends are, as seen on the following hardware:

```
distro: Ubuntu 25.04 x86_64 
kernel: 6.14.0-061400-generic 
model: 20FAS39200 ThinkPad T460s 
cpu: Intel i5-6300U (4) @ 3.000GHz 
memory: 2652MiB / 11367MiB 
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
|Baseline|`6.3`|`6.3`|`6.5`|`6.6`|`8.1`|`21.8`|`174.9`|`1665.9`|
|UTPs|`6.3`|`6.3`|`6.4`|`6.7`|`9.3`|`29.9`|`293.9`|`2719.7`|
|Amazon|`6.4`|`6.3`|`6.5`|`6.7`|`9.2`|`33.5`|`335.5`|`3106.5`|
|Google|`6.4`|`6.4`|`6.5`|`6.7`|`9.4`|`36.2`|`356.9`|`3543.5`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`565,175`|`565,175`|`572,601`|`574,506`|`624,463`|`1,091,418`|`3,271,321`|`3,696,585`|
|UTPs|`565,175`|`565,175`|`569,859`|`580,593`|`691,842`|`1,443,569`|`4,306,831`|`5,199,785`|
|Amazon|`564,967`|`564,967`|`574,394`|`592,182`|`700,408`|`1,418,098`|`1,530,804`|`2,171,344`|
|Google|`564,967`|`564,967`|`573,740`|`599,559`|`897,367`|`1,190,314`|`1,293,230`|`1,155,256`|

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
|Baseline|`8.7`|`8.8`|`8.2`|`9.0`|`9.4`|`22.4`|`147.1`|`1413.7`|
|UTPs|`7.9`|`8.8`|`8.8`|`8.6`|`11.3`|`34.6`|`257.2`|`2433.8`|
|Amazon|`8.0`|`8.8`|`8.2`|`9.2`|`12.4`|`44.0`|`363.6`|`3339.5`|
|Google|`8.5`|`8.7`|`9.0`|`8.4`|`12.9`|`51.3`|`405.7`|`3833.6`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`784,393`|`765,349`|`765,529`|`773,967`|`859,824`|`1,604,120`|`7,717,042`|`78,930,604`|
|UTPs|`784,393`|`784,905`|`771,451`|`801,194`|`1,067,225`|`2,705,129`|`22,501,242`|`179,560,257`|
|Amazon|`764,945`|`785,007`|`787,077`|`839,621`|`1,222,509`|`6,083,158`|`40,703,507`|`329,453,356`|
|Google|`764,945`|`765,623`|`775,687`|`871,665`|`1,493,647`|`6,127,203`|`41,481,908`|`605,350,320`|

## Site WebSocket

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.4`|`4.5`|`4.0`|`5.8`|`18.3`|`124.6`|`840.4`|`10032.4`|
|UTPs|`3.5`|`4.5`|`4.7`|`6.5`|`26.0`|`190.0`|`1404.8`|`14344.6`|
|Amazon|`3.5`|`4.1`|`4.5`|`6.7`|`23.9`|`163.8`|`1565.8`|`15392.0`|
|Google|`4.0`|`4.3`|`4.7`|`6.6`|`24.7`|`191.3`|`1652.2`|`15834.7`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`781,595`|`762,147`|`781,595`|`781,595`|`781,595`|`762,147`|`781,595`|`762,147`|
|UTPs|`762,147`|`762,147`|`781,595`|`762,147`|`762,147`|`762,147`|`762,147`|`762,147`|
|Amazon|`781,595`|`762,147`|`781,595`|`762,147`|`762,147`|`762,147`|`762,147`|`781,595`|
|Google|`762,531`|`781,595`|`762,147`|`781,595`|`762,147`|`762,147`|`762,147`|`781,595`|

