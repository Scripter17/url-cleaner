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
|Baseline|`6.3`|`6.3`|`6.3`|`6.6`|`8.4`|`26.1`|`228.4`|
|UTPs|`6.2`|`6.3`|`6.3`|`6.7`|`9.6`|`35.8`|`357.4`|
|Amazon|`6.2`|`6.3`|`6.4`|`6.7`|`9.8`|`38.9`|`395.1`|
|Google|`6.2`|`6.2`|`6.3`|`6.8`|`10.2`|`45.7`|`433.8`|

## CLI Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`503,786`|`497,795`|`511,351`|`517,067`|`574,911`|`1,110,591`|`2,694,393`|`4,371,929`|
|UTPs|`506,159`|`507,497`|`512,051`|`520,356`|`640,732`|`1,915,415`|`4,150,475`|`4,660,899`|
|Amazon|`504,587`|`507,637`|`513,451`|`534,205`|`764,649`|`888,323`|`1,104,443`|`1,882,794`|
|Google|`506,159`|`507,701`|`504,604`|`530,923`|`723,852`|`928,786`|`1,144,956`|`1,145,298`|

## Site HTTP Hyperfine

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`9.0`|`8.8`|`8.8`|`9.2`|`10.5`|`26.8`|`193.7`|
|UTPs|`9.1`|`8.9`|`9.1`|`9.1`|`11.9`|`37.7`|`285.2`|
|Amazon|`8.1`|`9.0`|`8.9`|`9.3`|`12.7`|`46.5`|`401.0`|
|Google|`8.7`|`8.8`|`9.1`|`9.3`|`13.5`|`52.7`|`447.9`|

## Site HTTP Massif

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`710,836`|`711,016`|`711,811`|`715,001`|`794,515`|`1,541,213`|`7,719,989`|`79,807,065`|
|UTPs|`710,836`|`711,122`|`711,896`|`743,763`|`941,249`|`2,604,205`|`22,503,210`|`180,397,133`|
|Amazon|`710,836`|`711,226`|`713,296`|`768,074`|`1,323,098`|`6,011,703`|`40,696,357`|`332,780,333`|
|Google|`710,836`|`711,290`|`713,936`|`817,338`|`1,582,682`|`5,991,291`|`41,479,003`|`605,829,463`|

## Site Websocket Hyperfine

Time it takes to do various amounts of the tasks, measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.7`|`4.7`|`5.1`|`9.4`|`57.1`|`525.8`|`5106.8`|
|UTPs|`4.6`|`4.3`|`5.0`|`9.1`|`57.5`|`526.0`|`5110.6`|
|Amazon|`4.7`|`4.7`|`5.2`|`9.7`|`56.2`|`508.6`|`4836.1`|
|Google|`4.8`|`4.7`|`5.0`|`9.8`|`57.0`|`514.5`|`4842.7`|

## Site Websocket Massif

Peak memory usage to do various amounts of the tasks, measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--:|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|
|UTPs|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|
|Amazon|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|
|Google|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|`707,829`|
