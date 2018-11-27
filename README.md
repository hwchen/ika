# Ika
ika is japanese for squid.

This software is based on the ideas of postgrest, in rust.

One goal is to explore the line between REST api and olap-type capabilities such as aggregation, based only on the metadata provided in postgres.

Here's a test `curl`, all query params are required currently:
```
curl "127.0.0.1:4000/test/pums/ztest_pums_5?select=st,agep&group_by=0&agg=sum.1"
```

Here's the current table for testing (to fit with the limited features so far):
```
ika=# select * from pums.ztest_pums_5;
    st     | agep
-----------+------
 matched   |   11
 matched   |   22
 unmatched |   33
(3 rows)

ika=# \d pums.ztest_pums_5
  Table "pums.ztest_pums_5"
 Column |  Type   | Modifiers
--------+---------+-----------
 st     | text    |
 agep   | integer |
 ```
