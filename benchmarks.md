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
|Baseline|1000|`7.5`|`8.4`|`10.0`|`0.5`|
|Baseline|10000|`19.4`|`26.0`|`34.9`|`3.5`|
|Baseline|100000|`185.9`|`223.8`|`275.6`|`28.0`|
|UTPs|1000|`8.6`|`9.6`|`12.1`|`0.4`|
|UTPs|10000|`28.8`|`35.8`|`51.9`|`4.7`|
|UTPs|100000|`296.1`|`356.3`|`448.4`|`49.3`|
|Amazon|1000|`8.4`|`9.4`|`12.6`|`0.7`|
|Amazon|10000|`30.1`|`38.2`|`51.5`|`5.6`|
|Amazon|100000|`319.9`|`394.4`|`562.1`|`86.2`|
|Google|1000|`8.9`|`9.9`|`11.9`|`0.7`|
|Google|10000|`35.6`|`43.6`|`59.2`|`5.2`|
|Google|100000|`341.3`|`414.1`|`460.7`|`34.6`|

## CLI Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1000|10000|100000|1000000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`504,430`|`506,429`|`513,208`|`519,869`|`614,535`|`1,528,803`|`4,472,491`|`6,635,885`|
|UTPs|`496,789`|`507,866`|`513,908`|`527,996`|`753,049`|`2,871,505`|`9,315,499`|`11,644,023`|
|Amazon|`504,214`|`508,006`|`515,986`|`537,432`|`865,313`|`2,974,418`|`3,522,006`|`3,771,524`|
|Google|`503,413`|`508,070`|`517,462`|`544,876`|`736,600`|`2,606,839`|`1,551,344`|`1,756,276`|

## Site HTTP Hyperfine

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|Count|Min|Mean|Max|Std. Dev.|
|:--:|:--:|--:|--:|--:|--:|
|Baseline|1000|`9.8`|`10.9`|`13.9`|`0.8`|
|Baseline|10000|`25.5`|`27.4`|`35.1`|`1.8`|
|Baseline|100000|`190.5`|`200.7`|`223.0`|`8.1`|
|UTPs|1000|`9.6`|`10.7`|`15.8`|`0.9`|
|UTPs|10000|`35.8`|`38.5`|`50.5`|`2.7`|
|UTPs|100000|`300.1`|`318.9`|`354.2`|`20.3`|
|Amazon|1000|`11.7`|`12.9`|`15.2`|`0.8`|
|Amazon|10000|`46.2`|`48.9`|`55.4`|`2.4`|
|Amazon|100000|`397.7`|`423.9`|`521.0`|`42.5`|
|Google|1000|`11.9`|`13.0`|`15.5`|`0.7`|
|Google|10000|`53.8`|`57.0`|`64.4`|`2.7`|
|Google|100000|`466.4`|`492.1`|`519.5`|`15.8`|

## Site HTTP Massif

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1000|10000|100000|1000000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`705,255`|`705,532`|`726,302`|`726,989`|`920,286`|`2,822,810`|`21,537,770`|`188,488,954`|
|UTPs|`705,403`|`705,226`|`719,834`|`760,552`|`1,176,990`|`4,898,738`|`45,216,970`|`408,568,932`|
|Amazon|`705,403`|`704,953`|`722,906`|`782,390`|`1,424,239`|`8,651,226`|`69,154,098`|`597,655,126`|
|Google|`705,403`|`704,953`|`723,965`|`807,746`|`1,751,455`|`9,603,938`|`75,554,098`|`930,090,834`|

## Site Websocket Hyperfine

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|Count|Min|Mean|Max|Std. Dev.|
|:--:|:--:|--:|--:|--:|--:|
|Baseline|1000|`35.2`|`36.4`|`42.1`|`0.9`|
|Baseline|10000|`61.5`|`66.8`|`75.9`|`3.0`|
|Baseline|100000|`291.1`|`310.4`|`354.5`|`18.4`|
|UTPs|1000|`35.5`|`36.9`|`38.8`|`0.8`|
|UTPs|10000|`69.6`|`81.2`|`118.9`|`10.1`|
|UTPs|100000|`430.9`|`477.8`|`526.9`|`37.4`|
|Amazon|1000|`35.2`|`38.5`|`44.2`|`1.5`|
|Amazon|10000|`76.8`|`87.5`|`104.5`|`6.1`|
|Amazon|100000|`502.9`|`573.8`|`620.7`|`46.8`|
|Google|1000|`37.8`|`39.1`|`40.9`|`0.7`|
|Google|10000|`88.4`|`98.1`|`112.9`|`6.2`|
|Google|100000|`585.4`|`604.0`|`660.5`|`23.4`|

## Site Websocket Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1000|10000|100000|1000000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`706,989`|`706,989`|`706,989`|`706,989`|`819,462`|`1,072,932`|`1,072,780`|`1,073,274`|
|UTPs|`706,989`|`706,989`|`706,989`|`735,636`|`978,156`|`978,156`|`978,156`|`978,156`|
|Amazon|`706,989`|`706,989`|`706,989`|`771,437`|`895,907`|`896,136`|`895,678`|`896,136`|
|Google|`706,989`|`706,989`|`706,989`|`783,715`|`893,650`|`893,357`|`893,650`|`893,943`|
