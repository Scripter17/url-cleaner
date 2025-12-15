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
|Baseline|`6.1`|`6.2`|`6.2`|`6.4`|`8.0`|`23.0`|`196.3`|
|UTPs|`6.2`|`6.2`|`6.3`|`6.7`|`9.2`|`31.3`|`283.9`|
|Amazon|`6.2`|`6.3`|`6.3`|`6.6`|`9.0`|`34.4`|`332.3`|
|Google|`6.2`|`6.2`|`6.4`|`6.6`|`9.4`|`38.0`|`379.8`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`514,052`|`515,072`|`518,252`|`520,447`|`570,463`|`1,037,993`|`2,872,666`|`3,994,850`|
|UTPs|`514,052`|`515,142`|`518,952`|`527,910`|`639,639`|`1,727,975`|`5,130,544`|`5,174,936`|
|Amazon|`514,052`|`515,282`|`520,352`|`541,106`|`819,566`|`815,803`|`1,157,269`|`1,349,993`|
|Google|`514,052`|`515,346`|`520,992`|`548,062`|`695,873`|`894,055`|`954,400`|`1,054,899`|

## Site HTTP

The max payload was increased from 25MiB to 1GiB.

While a million of the baseline task does fit in the 25MiB, the rest of the extreme numbers don't happen.

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`8.6`|`8.3`|`8.4`|`8.5`|`10.0`|`21.9`|`144.9`|
|UTPs|`8.8`|`8.2`|`8.5`|`8.6`|`10.6`|`33.8`|`245.6`|
|Amazon|`7.4`|`8.4`|`8.5`|`8.9`|`12.2`|`41.2`|`329.3`|
|Google|`7.7`|`8.3`|`8.6`|`8.8`|`12.9`|`48.4`|`402.2`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`721,119`|`721,523`|`721,703`|`730,517`|`798,156`|`1,540,198`|`7,644,094`|`78,809,794`|
|UTPs|`701,671`|`702,183`|`726,920`|`757,153`|`1,004,335`|`2,611,806`|`22,431,484`|`179,556,888`|
|Amazon|`721,119`|`702,285`|`711,496`|`775,893`|`1,314,095`|`6,066,201`|`40,988,829`|`332,001,512`|
|Google|`701,671`|`704,525`|`712,760`|`808,391`|`1,429,493`|`6,000,710`|`41,410,560`|`602,274,509`|

## Site WebSocket

### Speed

Measured in milliseconds.

|Name|0|1|10|100|1,000|10,000|100,000|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`4.4`|`4.3`|`4.3`|`4.0`|`6.2`|`21.5`|`158.7`|
|UTPs|`4.4`|`4.5`|`4.5`|`4.9`|`7.6`|`36.1`|`305.0`|
|Amazon|`4.3`|`4.4`|`4.5`|`4.7`|`9.6`|`43.5`|`394.9`|
|Google|`4.3`|`4.5`|`4.4`|`5.2`|`10.6`|`51.3`|`475.0`|

### Peak memory usage

Measured in bytes.

|Name|0|1|10|100|1,000|10,000|100,000|1,000,000|
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`718,313`|`718,313`|`698,865`|`718,313`|`792,107`|`908,090`|`904,014`|`904,014`|
|UTPs|`698,865`|`699,249`|`698,865`|`713,808`|`810,496`|`825,872`|`825,872`|`825,872`|
|Amazon|`718,313`|`718,313`|`718,313`|`758,192`|`820,798`|`820,862`|`820,862`|`820,862`|
|Google|`698,865`|`718,313`|`698,865`|`770,992`|`820,358`|`820,422`|`820,358`|`820,422`|

