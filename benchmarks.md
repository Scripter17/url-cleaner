# Benchmarks

As measured on a thinkpad T460S (from 2016) running Kubuntu.

## Tasks

The tasks that are benchmarks

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
|Baseline|`6.2`|`6.2`|`6.2`|`6.4`|`8.3`|`24.9`|`223.7`|
|UTPs|`6.1`|`6.2`|`6.2`|`6.6`|`9.2`|`33.7`|`321.6`|
|Amazon|`6.1`|`6.1`|`6.2`|`6.5`|`9.2`|`36.6`|`360.0`|
|Google|`6.1`|`6.2`|`6.2`|`6.6`|`9.9`|`42.8`|`408.6`|

## CLI Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`502,897`|`504,165`|`508,089`|`510,324`|`568,244`|`1,109,705`|`2,973,047`|`3,936,995`|
|UTPs|`502,897`|`504,235`|`508,789`|`517,303`|`637,560`|`1,719,309`|`4,294,597`|`4,725,779`|
|Amazon|`502,897`|`504,375`|`510,306`|`531,151`|`808,015`|`978,883`|`1,440,827`|`1,327,197`|
|Google|`502,897`|`502,905`|`511,402`|`537,848`|`808,906`|`910,228`|`1,051,218`|`1,161,526`|

## Site HTTP Hyperfine

Time it takes to do various amounts of the tasks, measured in milliseconds.

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`8.5`|`8.9`|`8.9`|`9.1`|`10.5`|`24.4`|`166.2`|
|UTPs|`8.4`|`8.8`|`8.8`|`9.3`|`11.7`|`35.7`|`278.4`|
|Amazon|`9.0`|`8.3`|`9.1`|`9.1`|`12.7`|`44.7`|`375.3`|
|Google|`8.5`|`9.0`|`8.8`|`9.3`|`13.5`|`52.1`|`441.0`|

## Site HTTP Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`707,294`|`707,474`|`708,910`|`716,856`|`802,272`|`1,546,860`|`7,650,004`|`78,623,272`|
|UTPs|`707,294`|`707,580`|`713,980`|`743,110`|`1,009,380`|`2,617,343`|`22,436,965`|`179,561,670`|
|Amazon|`707,294`|`707,684`|`716,895`|`764,532`|`1,319,556`|`6,075,893`|`40,643,993`|`328,663,034`|
|Google|`707,294`|`707,748`|`721,423`|`813,796`|`1,435,371`|`6,008,034`|`41,418,971`|`603,300,698`|

## Site WebSocket Hyperfine

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.6`|`4.7`|`4.8`|`9.6`|`55.5`|`520.7`|`5018.8`|
|UTPs|`4.6`|`4.5`|`4.9`|`9.6`|`57.4`|`505.6`|`5059.2`|
|Amazon|`4.7`|`4.6`|`5.1`|`9.9`|`57.0`|`511.0`|`4875.1`|
|Google|`4.7`|`4.6`|`4.6`|`9.2`|`56.9`|`506.6`|`4804.9`|

## Site WebSocket Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|
|UTPs|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|
|Amazon|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|
|Google|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|`704,071`|

