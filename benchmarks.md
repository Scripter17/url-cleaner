# Benchmarks

## As seen on

```
distro: Ubuntu 26.04.1 LTS x86_64 
kernel: 7.0.0-31-generic 
model: 20FAS39200 ThinkPad T460s 
cpu: Intel i5-6300U (4) @ 3.000GHz 
memory: 2585MiB / 11362MiB 
```

## Tasks

|Name|Task|ParamsDiff|
|:--|:--|:--|
|Baseline|`https://example.com`|None|
|UTPs    |`https://example.com?fb_action_ids&mc_eid&ml_subscriber_hash&oft_ck&s_cid&unicorn_click_id`|None|
|Amazon  |`https://www.amazon.ca/UGREEN-Charger-Compact-Adapter-MacBook/dp/B0C6DX66TN/ref=sr_1_5?crid=2CNEQ7A6QR5NM&keywords=ugreen&qid=1704364659&sprefix=ugreen%2Caps%2C139&sr=8-5&ufe=app_do%3Aamzn1.fos.b06bdbbe-20fd-4ebc-88cf-fa04f1ca0da8`|None|
|Google  |`https://www.google.com/search?q=url+cleaner&sca_esv=de6549fe37924183&ei=eRAYabb6O7Gb4-EP79Xe6A8&ved=0ahUKEwj2mqWLt_OQAxWxzTgGHe-qF_0Q4dUDCBE&oq=url+cleaner&gs_lp=Egxnd3Mtd2l6LXNlcnAiC3VybCBjbGVhbmVySABQAFgAcAB4AZABAJgBAKABAKoBALgBDMgBAJgCAKACAJgDAJIHAKAHALIHALgHAMIHAMgHAA&sclient=gws-wiz-serp`|None|

## Tables

### CLI - Hyperfine Time

The average number of milliseconds.

|Task|`0`|`1`|`10`|`100`|`1,000`|`10,000`|`100,000`|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`5.7`|`5.9`|`6.0`|`6.1`|`7.4`|`15.4`|`91.5`| <!-- + 19.71s -> 19.71s -->
|UTPs    |`5.8`|`5.9`|`6.0`|`6.3`|`8.2`|`18.8`|`119.9`| <!-- + 21.38s -> 41.09s -->
|Amazon  |`5.7`|`5.9`|`6.0`|`6.4`|`8.5`|`20.8`|`131.5`| <!-- + 20.71s -> 61.81s -->
|Google  |`5.8`|`6.0`|`6.1`|`6.5`|`8.9`|`23.1`|`154.8`| <!-- + 21.20s -> 83.01s -->

### CLI - Hyperfine Memory

The average peak resident memory usage.

Please note that, as far as I understand, the vast majority the base value (num=0) is shared memory, meaning it's either shared libraries that other programs can use without additional allocations or unused heap space that can be given to other programs as needed.

I may be wrong about that though.

|Task|`0`|`1`|`10`|`100`|`1,000`|`10,000`|`100,000`|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`11,346,121`|`11,940,967`|`11,896,537`|`11,888,058`|`11,930,383`|`12,563,692`|`13,226,135`| <!-- + 20.09s -> 103.10s -->
|UTPs    |`11,337,928`|`11,929,459`|`11,916,403`|`11,904,328`|`12,231,766`|`13,213,589`|`13,988,693`| <!-- + 21.12s -> 124.22s -->
|Amazon  |`11,318,948`|`11,854,452`|`11,851,575`|`11,834,916`|`12,481,939`|`13,675,081`|`14,584,100`| <!-- + 21.33s -> 145.55s -->
|Google  |`11,250,232`|`11,855,303`|`11,862,149`|`11,882,331`|`12,561,236`|`13,961,589`|`14,995,024`| <!-- + 21.24s -> 166.80s -->

### CLI - Massif

The peak heap usage.

|Task|`0`|`1`|`10`|`100`|`1,000`|`10,000`|`100,000`|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`674,386`|`674,786`|`676,719`|`684,036`|`777,360`|`1,212,415`|`1,621,791`| <!-- + 29.77s -> 196.56s -->
|UTPs    |`673,666`|`675,652`|`678,336`|`704,622`|`926,652`|`1,623,503`|`1,854,528`| <!-- + 28.56s -> 225.12s -->
|Amazon  |`674,578`|`676,347`|`678,040`|`728,048`|`1,175,835`|`1,702,041`|`1,929,782`| <!-- + 29.54s -> 254.66s -->
|Google  |`673,666`|`676,635`|`685,708`|`787,039`|`952,963`|`1,693,464`|`1,823,270`| <!-- + 30.06s -> 284.72s -->

### Site - HTTP - Massif

The peak heap usage.

|Task|`0`|`1`|`10`|`100`|`1,000`|`10,000`|`100,000`|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|`712,128`|`732,996`|`733,988`|`711,824`|`779,197`|`1,159,537`|`2,085,439`| <!-- + 32.42s -> 317.14s -->
|UTPs    |`734,788`|`734,484`|`711,267`|`726,238`|`996,928`|`1,907,758`|`2,307,043`| <!-- + 32.06s -> 349.19s -->
|Amazon  |`711,267`|`711,328`|`710,944`|`740,474`|`1,133,878`|`2,486,998`|`2,721,629`| <!-- + 32.60s -> 381.80s -->
|Google  |`734,788`|`734,788`|`734,484`|`739,902`|`1,147,710`|`2,894,926`|`2,992,625`| <!-- + 33.15s -> 414.95s -->

### Site - HTTPS - Massif

The peak heap usage.

|Task|`0`|`1`|`10`|`100`|`1,000`|`10,000`|`100,000`|
|:--|--:|--:|--:|--:|--:|--:|--:|
|Baseline|