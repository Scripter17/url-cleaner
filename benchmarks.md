# Benchmarks

As measured on a thinkpad T460s (from 2016) running Kubuntu.

## Benchmarks

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
|Baseline|`6.3`|`6.2`|`6.2`|`6.5`|`8.5`|`26.9`|`192.8`|`1826.1`|
|UTPs|`6.1`|`6.2`|`6.3`|`6.7`|`9.2`|`31.6`|`268.0`|`2542.7`|
|Amazon|`6.1`|`6.2`|`6.3`|`6.7`|`9.5`|`35.3`|`316.9`|`3072.5`|
|Google|`6.1`|`6.2`|`6.3`|`6.7`|`9.7`|`37.7`|`356.6`|`3428.3`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`535,354`|`535,354`|`542,988`|`541,929`|`594,850`|`1,061,954`|`5,426,058`|`44,821,208`|
|UTPs|`535,562`|`536,644`|`540,038`|`551,998`|`656,617`|`1,758,127`|`7,444,538`|`46,617,080`|
|Amazon|`535,562`|`536,784`|`541,646`|`564,817`|`820,238`|`1,574,470`|`6,750,823`|`58,736,163`|
|Google|`535,554`|`535,562`|`546,267`|`568,940`|`867,169`|`1,739,107`|`6,585,674`|`57,673,292`|

## Site HTTP

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`8.1`|`8.2`|`8.1`|`8.1`|`9.3`|`21.8`|`145.5`|`1335.8`|
|UTPs|`7.3`|`8.3`|`8.1`|`8.3`|`10.0`|`33.8`|`253.3`|`2439.4`|
|Amazon|`7.3`|`8.0`|`8.3`|`8.2`|`11.7`|`42.5`|`349.1`|`3286.4`|
|Google|`8.0`|`8.1`|`8.3`|`8.5`|`12.7`|`50.2`|`410.6`|`3983.1`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`753,566`|`753,970`|`754,150`|`762,608`|`829,283`|`1,571,671`|`7,673,279`|`78,839,611`|
|UTPs|`753,566`|`735,014`|`741,828`|`789,510`|`1,036,398`|`2,674,302`|`22,468,499`|`179,483,147`|
|Amazon|`734,118`|`734,732`|`744,239`|`788,993`|`1,346,542`|`6,084,888`|`40,636,892`|`329,374,543`|
|Google|`753,566`|`754,244`|`744,996`|`860,286`|`1,606,062`|`6,056,138`|`41,447,612`|`604,907,024`|

## Site WebSocket

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`3.9`|`4.2`|`4.2`|`5.7`|`21.0`|`125.2`|`855.2`|`9409.1`|
|UTPs|`3.4`|`4.3`|`4.5`|`6.0`|`22.1`|`179.5`|`1377.7`|`14026.0`|
|Amazon|`3.3`|`4.3`|`4.4`|`6.4`|`24.4`|`151.4`|`1476.1`|`14437.4`|
|Google|`3.6`|`4.3`|`4.5`|`6.7`|`24.0`|`181.2`|`1571.6`|`15729.4`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`750,768`|`750,768`|`750,768`|`731,320`|`731,320`|`750,768`|`750,768`|`731,320`|
|UTPs|`731,320`|`750,768`|`731,320`|`750,768`|`731,320`|`750,768`|`750,768`|`731,320`|
|Amazon|`731,320`|`750,768`|`750,768`|`750,768`|`731,704`|`731,320`|`750,768`|`750,768`|
|Google|`731,320`|`750,768`|`731,320`|`750,768`|`731,320`|`750,768`|`731,320`|`750,768`|

