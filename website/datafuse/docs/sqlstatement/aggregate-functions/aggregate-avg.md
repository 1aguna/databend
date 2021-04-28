---
id: aggregate-avg
title: AVG
---

Aggregate function.

The AVG() function returns the average value of an expression.

**Note:** NULL values are not counted.

## Syntax

```sql
AVG(expression)
```

## Arguments

| Arguments   | Description |
| ----------- | ----------- |
| expression  | Any numerical expression |

## Return Type

double

## Examples

```sql
mysql> SELECT AVG(*) FROM numbers(3);
+--------+
| avg(*) |
+--------+
|      1 |
+--------+

mysql> SELECT AVG(number) FROM numbers(3);
+-------------+
| avg(number) |
+-------------+
|           1 |
+-------------+

mysql> SELECT AVG(number+1) FROM numbers(3);
+----------------------+
| avg(plus(number, 1)) |
+----------------------+
|                    2 |
+----------------------+

mysql> SELECT AVG(number+1) AS a FROM numbers(3);
+------+
| a    |
+------+
|    2 |
+------+
```
