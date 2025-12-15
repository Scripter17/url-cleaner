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

|Name|0|1|10|100|1,000|10,000|100,000|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`6.1`|`6.2`|`6.3`|`6.5`|`8.1`|`23.3`|`197.5`|
|UTPs|`6.1`|`6.2`|`6.3`|`6.6`|`9.3`|`32.5`|`317.6`|
|Amazon|`6.2`|`6.2`|`6.3`|`6.6`|`9.1`|`34.5`|`370.4`|
|Google|`6.2`|`6.2`|`6.3`|`6.7`|`9.6`|`41.1`|`432.5`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`514,060`|`514,060`|`518,252`|`520,427`|`572,432`|`1,038,132`|`2,904,706`|`3,786,490`|
|UTPs|`514,060`|`515,142`|`518,952`|`527,052`|`640,052`|`1,739,988`|`4,411,568`|`4,627,856`|
|Amazon|`514,052`|`515,282`|`520,352`|`541,106`|`844,138`|`1,163,573`|`1,956,110`|`1,251,222`|
|Google|`514,468`|`515,346`|`520,992`|`547,854`|`687,105`|`1,119,153`|`1,096,154`|`1,276,150`|

## Site HTTP

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`8.6`|`8.6`|`8.5`|`8.6`|`9.8`|`22.2`|`149.9`|
|UTPs|`7.6`|`8.4`|`8.7`|`8.6`|`11.0`|`33.7`|`248.6`|
|Amazon|`7.3`|`8.4`|`8.4`|`8.4`|`11.7`|`42.4`|`352.2`|
|Google|`8.5`|`8.4`|`8.4`|`8.5`|`13.0`|`51.5`|`418.3`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`701,671`|`702,075`|`704,392`|`730,181`|`796,836`|`1,540,370`|`7,646,306`|`76,636,805`|
|UTPs|`721,119`|`702,567`|`726,215`|`740,957`|`1,003,951`|`2,611,611`|`22,431,468`|`179,555,944`|
|Amazon|`721,119`|`721,906`|`730,944`|`759,511`|`1,159,527`|`6,017,923`|`40,970,965`|`328,511,135`|
|Google|`721,055`|`702,349`|`712,549`|`808,775`|`1,429,687`|`6,001,441`|`41,868,030`|`606,488,545`|

## Site WebSocket

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.5`|`4.3`|`4.6`|`5.9`|`20.2`|`122.3`|`819.0`|
|UTPs|`3.5`|`4.5`|`4.6`|`6.4`|`23.2`|`148.5`|`1428.4`|
|Amazon|`3.7`|`4.3`|`4.7`|`6.4`|`26.0`|`169.6`|`1481.8`|
|Google|`3.6`|`4.5`|`4.4`|`6.4`|`26.4`|`166.3`|`1637.6`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`698,865`|`699,249`|`698,865`|`698,865`|`698,865`|`718,313`|`699,249`|`718,313`|
|UTPs|`718,313`|`718,313`|`698,865`|`698,865`|`698,865`|`698,865`|`718,313`|`699,249`|
|Amazon|`718,313`|`718,313`|`698,865`|`718,313`|`698,865`|`699,249`|`698,865`|`718,313`|
|Google|`698,865`|`698,865`|`718,313`|`699,249`|`718,313`|`718,313`|`698,865`|`699,249`|

