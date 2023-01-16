### postgres int4range范围数据结构的使用
(x,y)括号表示不包含，[x,y]表示包含关系
```sql
--- 查询这两个区间有没有交集
select int4range(1, 10) && int4range(12, 20);
--- 查看一个数是不是在这个区间之内
select int4range(1, 10) @> 12;
```

* 首次创建gist索引会报错需要创建
```sql
    CREATE EXTENSION btree_gist;
```
