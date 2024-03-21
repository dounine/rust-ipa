
数据库迁移转换
```sql
select appid    as app_id,
       country,
       name,
       lname    as origin_name,
       bundleId as bundle_id,
       des,
       icon,
       platform,
       price,
       genres,
       single,
       time     as created_at
from appStore;
select appid as app_id,
       country,
       version,
       des,
       download,
       official,
       file  as download_url,
       size,
       time  as created_at
from versionStore;
select appid    as app_id,
       country,
       version,
       name,
       icon,
       link,
       bundleId as bundle_id,
       size,
       price,
       status,
       time     as created_at
from dumpStore;

select id,
       userid  as user_id,
       money,
       coin,
       trade_no,
       platform,
       paytime as payed_time,
       pay     as payed,
       time    as created_at
from payStore;
select id,
       userid                                                                                                                                                   as user_id,
       coin,
       des,
       -- 判断des多个条件,包含"购买":0,"提取":1,"下载":2,"赠送":3,"收到":4,"退款":5
       if(des like '%购买%', 0, if(des like '%提取%', 1, if(des like '%下载%', 2, if(des like '%赠送%', 3,
                                                                                     if(des like '%收到%', 4, if(des like '%退款%', 5, if(coin > 0, 0, 5))))))) as record_type,
       time                                                                                                                                                     as created_at
from coinStore;
select id,
       nickName                                        as nick_name,
       null                                            as user_name,
       mail                                            as email,
       null                                            as password,
       ccode                                           as channel_code,
       registerIp                                      as ip,
       uid,
       headUrl                                         as avatar,
       status,
       if(mail is not null, 0, if(platform = 0, 1, 2)) as platform,
       0                                               as user_type,
       time                                            as created_at
from userStore;
select userid as user_id,
       appid  as app_id,
       country,
       version,
       time   as created_at
from userDump;
```