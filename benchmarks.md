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

|Name|0|1|10|100|1,000|10,000|100,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`6.1`|`6.2`|`6.2`|`6.4`|`8.4`|`25.6`|`234.4`|
|UTPs|`6.1`|`6.2`|`6.3`|`6.5`|`9.5`|`35.7`|`387.4`|
|Amazon|`6.1`|`6.2`|`6.3`|`6.5`|`9.5`|`38.3`|`377.1`|
|Google|`6.1`|`6.2`|`6.3`|`6.6`|`10.1`|`44.6`|`441.0`|

## CLI Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`505,394`|`506,870`|`511,002`|`513,205`|`570,969`|`1,068,562`|`3,411,284`|`3,478,636`|
|UTPs|`505,610`|`505,610`|`511,494`|`519,799`|`640,385`|`1,560,436`|`4,643,118`|`4,571,500`|
|Amazon|`505,602`|`507,080`|`515,163`|`533,654`|`782,024`|`1,075,718`|`1,823,551`|`1,274,415`|
|Google|`505,610`|`505,610`|`513,534`|`540,396`|`721,780`|`834,908`|`960,952`|`1,396,078`|

## Site HTTP Hyperfine

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`8.4`|`8.4`|`8.9`|`9.2`|`10.5`|`26.2`|`172.4`|
|UTPs|`8.4`|`8.7`|`8.8`|`8.6`|`11.6`|`37.2`|`304.4`|
|Amazon|`8.7`|`8.9`|`8.7`|`9.2`|`12.4`|`46.8`|`394.4`|
|Google|`8.3`|`8.5`|`8.8`|`9.2`|`13.5`|`52.9`|`476.1`|

## Site HTTP Massif

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`710,948`|`711,128`|`713,690`|`720,170`|`806,174`|`1,548,381`|`7,655,125`|`78,355,650`|
|UTPs|`710,948`|`711,234`|`715,806`|`747,140`|`1,013,034`|`2,619,516`|`22,442,140`|`179,455,228`|
|Amazon|`710,948`|`711,338`|`720,965`|`766,079`|`1,323,210`|`6,027,431`|`40,622,386`|`328,522,461`|
|Google|`710,948`|`711,402`|`721,189`|`817,450`|`1,582,794`|`6,011,712`|`41,420,776`|`604,375,283`|

## Site Websocket Hyperfine

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.4`|`4.6`|`4.8`|`9.3`|`54.8`|`534.2`|`5060.1`|
|UTPs|`4.6`|`4.2`|`5.0`|`9.8`|`56.9`|`506.5`|`5092.7`|
|Amazon|`4.7`|`4.6`|`5.0`|`9.8`|`57.1`|`513.2`|`4878.1`|
|Google|`4.5`|`4.2`|`4.9`|`9.7`|`56.6`|`509.5`|`4783.5`|

## Site Websocket Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|
|UTPs|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|
|Amazon|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|
|Google|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|`707,677`|
