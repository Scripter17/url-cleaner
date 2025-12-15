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
|Baseline|`6.1`|`6.2`|`6.3`|`6.3`|`8.0`|`23.1`|`203.2`|
|UTPs|`6.1`|`6.2`|`6.2`|`6.6`|`9.2`|`30.9`|`290.0`|
|Amazon|`6.2`|`6.2`|`6.4`|`6.5`|`9.0`|`33.6`|`357.2`|
|Google|`6.2`|`6.2`|`6.3`|`6.6`|`9.5`|`38.1`|`379.9`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`514,052`|`515,072`|`518,252`|`520,427`|`570,463`|`1,036,993`|`2,759,386`|`3,735,782`|
|UTPs|`514,052`|`515,142`|`519,971`|`527,356`|`638,709`|`1,735,089`|`4,325,424`|`4,593,958`|
|Amazon|`514,052`|`515,282`|`520,352`|`541,106`|`768,570`|`1,302,563`|`1,212,630`|`2,900,175`|
|Google|`514,052`|`515,346`|`520,992`|`547,854`|`846,460`|`1,361,149`|`942,391`|`1,466,792`|

## Site HTTP

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`9.3`|`9.4`|`9.2`|`8.4`|`9.8`|`22.4`|`140.4`|
|UTPs|`7.3`|`8.3`|`8.3`|`8.3`|`10.6`|`34.0`|`243.9`|
|Amazon|`7.3`|`8.4`|`8.6`|`8.7`|`12.1`|`41.1`|`334.9`|
|Google|`7.5`|`8.5`|`8.3`|`8.7`|`12.4`|`48.8`|`399.2`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`721,119`|`702,459`|`708,014`|`710,653`|`797,546`|`1,540,618`|`7,646,906`|`78,811,040`|
|UTPs|`721,119`|`702,183`|`707,472`|`757,569`|`1,004,335`|`2,610,968`|`22,431,664`|`179,555,964`|
|Amazon|`721,119`|`702,285`|`731,024`|`776,069`|`1,314,095`|`6,018,946`|`41,157,737`|`330,903,770`|
|Google|`701,671`|`702,349`|`733,499`|`808,775`|`1,429,906`|`6,010,052`|`41,128,642`|`616,922,372`|

## Site WebSocket

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.4`|`4.3`|`4.6`|`5.7`|`21.3`|`122.1`|`859.1`|
|UTPs|`3.4`|`4.4`|`4.5`|`6.2`|`22.7`|`148.8`|`1387.5`|
|Amazon|`3.6`|`4.4`|`4.6`|`6.0`|`24.2`|`167.3`|`1473.6`|
|Google|`3.6`|`4.4`|`4.6`|`6.7`|`26.5`|`164.9`|`1569.5`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`698,865`|`718,313`|`718,313`|`712,144`|`718,313`|`698,865`|`718,313`|`718,313`|
|UTPs|`718,313`|`698,865`|`718,313`|`718,313`|`698,865`|`698,865`|`699,249`|`718,313`|
|Amazon|`698,865`|`699,249`|`718,313`|`718,313`|`699,249`|`698,865`|`699,249`|`718,313`|
|Google|`718,313`|`718,313`|`698,865`|`698,865`|`698,865`|`718,313`|`718,313`|`698,865`|

