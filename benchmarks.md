# Benchmarks

## Tasks

|Name|Description|URL|
|:--:|:--:|:--|
|Baseline|An already clean URL|`https://example.com`|
|UTPs|Baseline with some universal tracking parameters|`https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id`|
|Amazon|An amazon product listing|`https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8`|
|Google|A google search result|`https://www.google.com/search?q=url+cleaner&sca_esv=de6549fe37924183&ei=eRAYabb6O7Gb4-EP79Xe6A8&ved=0ahUKEwj2mqWLt_OQAxWxzTgGHe-qF_0Q4dUDCBE&oq=url+cleaner&gs_lp=Egxnd3Mtd2l6LXNlcnAiC3VybCBjbGVhbmVySABQAFgAcAB4AZABAJgBAKABAKoBALgBDMgBAJgCAKACAJgDAJIHAKAHALIHALgHAMIHAMgHAA&sclient=gws-wiz-serp`|

## CLI Hyperfine

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|Count|Min|Mean|Max|Std. Dev.|
|:--:|:--:|--:|--:|--:|--:|
|Baseline|1000|`7.4`|`8.3`|`9.8`|`0.6`|
|Baseline|10000|`19.1`|`27.0`|`39.9`|`4.5`|
|Baseline|100000|`190.5`|`225.2`|`260.5`|`24.8`|
|UTPs|1000|`8.4`|`9.6`|`10.7`|`0.4`|
|UTPs|10000|`28.6`|`37.1`|`51.2`|`5.2`|
|UTPs|100000|`296.4`|`347.6`|`419.6`|`44.0`|
|Amazon|1000|`8.6`|`9.5`|`11.7`|`0.7`|
|Amazon|10000|`31.0`|`39.8`|`61.2`|`5.3`|
|Amazon|100000|`336.8`|`374.9`|`419.6`|`25.7`|
|Google|1000|`9.0`|`10.1`|`12.1`|`0.7`|
|Google|10000|`38.3`|`45.2`|`58.5`|`5.3`|
|Google|100000|`377.2`|`451.4`|`522.7`|`41.7`|

## CLI Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1000|10000|100000|1000000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`499,800`|`503,166`|`509,382`|`523,060`|`644,449`|`1,843,609`|`6,914,451`|`7,984,015`|
|UTPs|`492,159`|`503,633`|`512,869`|`527,334`|`783,655`|`3,021,240`|`12,645,015`|`13,846,837`|
|Amazon|`499,584`|`503,376`|`514,588`|`535,210`|`875,047`|`1,950,047`|`2,585,856`|`3,366,252`|
|Google|`499,800`|`503,440`|`515,021`|`541,806`|`898,958`|`4,052,223`|`1,822,181`|`4,266,785`|

## Site HTTP Hyperfine

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|Count|Min|Mean|Max|Std. Dev.|
|:--:|:--:|--:|--:|--:|--:|
|Baseline|1000|`10.0`|`11.0`|`16.9`|`0.9`|
|Baseline|10000|`25.7`|`27.8`|`38.1`|`2.2`|
|Baseline|100000|`194.0`|`209.2`|`255.4`|`17.0`|
|UTPs|1000|`10.3`|`11.2`|`13.2`|`0.7`|
|UTPs|10000|`35.6`|`37.4`|`47.5`|`2.1`|
|UTPs|100000|`291.4`|`307.1`|`337.7`|`17.3`|
|Amazon|1000|`11.2`|`12.2`|`14.4`|`0.7`|
|Amazon|10000|`44.1`|`46.3`|`57.4`|`3.1`|
|Amazon|100000|`374.6`|`402.5`|`484.2`|`37.0`|
|Google|1000|`12.1`|`13.4`|`15.8`|`0.6`|
|Google|10000|`52.0`|`54.4`|`63.9`|`2.6`|
|Google|100000|`450.2`|`476.2`|`558.4`|`33.9`|

## Site HTTP Massif

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1000|10000|100000|1000000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`700,112`|`708,522`|`714,352`|`725,115`|`919,118`|`2,818,048`|`21,930,910`|`200,633,234`|
|UTPs|`700,641`|`700,464`|`725,754`|`760,181`|`1,207,214`|`5,213,222`|`48,481,504`|`439,639,841`|
|Amazon|`700,112`|`700,191`|`721,065`|`781,191`|`1,456,158`|`8,980,460`|`70,232,213`|`620,292,566`|
|Google|`700,641`|`709,256`|`721,187`|`804,489`|`1,781,504`|`9,599,176`|`76,694,875`|`924,839,990`|

## Site Websocket Hyperfine

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|Count|Min|Mean|Max|Std. Dev.|
|:--:|:--:|--:|--:|--:|--:|
|Baseline|1000|`41.0`|`44.8`|`53.7`|`5.3`|
|Baseline|10000|`70.1`|`73.9`|`109.9`|`8.2`|
|Baseline|100000|`86.5`|`87.9`|`102.9`|`2.7`|
|UTPs|1000|`40.5`|`45.3`|`51.0`|`4.6`|
|UTPs|10000|`45.0`|`45.8`|`54.6`|`1.4`|
|UTPs|100000|`379.6`|`385.0`|`391.2`|`4.6`|
|Amazon|1000|`34.8`|`37.1`|`40.7`|`2.1`|
|Amazon|10000|`59.4`|`60.9`|`64.9`|`1.6`|
|Amazon|100000|`2190.2`|`2194.4`|`2198.2`|`2.9`|
|Google|1000|`34.3`|`35.9`|`38.3`|`1.6`|
|Google|10000|`71.8`|`73.5`|`76.7`|`1.6`|
|Google|100000|`3555.7`|`3560.7`|`3568.3`|`3.8`|

## Site Websocket Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1000|10000|100000|1000000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`702,227`|`702,227`|`702,227`|`702,227`|`754,354`|`878,194`|`878,194`|`878,194`|
|UTPs|`702,227`|`702,227`|`702,227`|`715,970`|`878,194`|`878,194`|`878,194`|`878,194`|
|Amazon|`702,227`|`702,227`|`702,227`|`760,354`|`878,194`|`878,194`|`878,194`|`878,194`|
|Google|`702,227`|`702,227`|`702,227`|`773,154`|`878,194`|`878,194`|`878,194`|`878,194`|
