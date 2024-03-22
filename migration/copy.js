const mysql_knex = require('knex')({
    client: 'mysql',
    connection: {
        host: 'localhost',
        user: 'root',
        password: 'root',
        database: 'dump',
        timezone: 'PRC'
    },
    pool: {
        min: 3,
        max: 100
    },
    useNullAsDefault: true
})
const postgres_knex = require('knex')({
    client: 'postgresql',
    connection: {
        host: '127.0.0.1',
        user: 'postgres',
        port: 5432,
        password: 'postgres',
        database: 'rust-ipadump',
        timezone: 'PRC'
    },
    pool: {
        min: 3,
        max: 100
    },
    migrations: {
        tableName: "seaql_migrations",
    },
    useNullAsDefault: true
})

const copy_pay = async () => {
    let list = await mysql_knex.raw(`
        select id,
           userid                   as user_id,
           money,
           coin,
           trade_no,
           platform,
           paytime                  as payed_time,
           if(pay = 1, true, false) as payed,
           time                     as created_at
        from payStore
    `)
        .then(res => res[0])
    await postgres_knex("pay").truncate()
    let batch = 100
    for (let i = 0; i < list.length; i += batch) {
        let list_batch = list.slice(i, i + batch).map(item => {
            return {
                id: item.id,
                user_id: item.user_id,
                money: item.money,
                coin: item.coin,
                trade_no: item.trade_no,
                platform: item.platform,
                payed_time: item.payed_time,
                payed: item.payed,
                created_at: item.created_at
            }
        })
        await postgres_knex.batchInsert('pay', list_batch, batch)
    }
    console.log('pay表数据迁移完成')
}
const copy_pay_record = async () => {
    let list = await mysql_knex.raw(`
        select id,
               userid                                                                                                                                                   as user_id,
               coin,
               des,
               if(des like '购买%', 0, if(des like '提取%', 1, if(des like '下载%', 2, if(des like '赠送%', 3,
                                                                                             if(des like '收到%', 4, if(des like '退款%', 5, if(coin > 0, 0, 5))))))) as record_type,
               time                                                                                                                                                     as created_at
        from coinStore
    `)
        .then(res => res[0])
    await postgres_knex("pay_record").truncate()
    let batch = 100
    for (let i = 0; i < list.length; i += batch) {
        let list_batch = list.slice(i, i + batch).map(item => {
            let record_type = -1;
            if (item.des.indexOf('购买') !== -1) {
                record_type = 0
            } else if (item.des.indexOf('提取') !== -1) {
                record_type = 1
            } else if (item.des.indexOf('下载') !== -1) {
                record_type = 2
            } else if (item.des.indexOf('赠送') !== -1) {
                record_type = 3
            } else if (item.des.indexOf('收到') !== -1) {
                record_type = 4
            } else if (item.des.indexOf('退款') !== -1) {
                record_type = 5
            }
            if (record_type === -1) {
                if (item.coin > 0) {
                    record_type = 0
                } else {
                    record_type = 5
                }
            }
            return {
                id: item.id,
                user_id: item.user_id,
                coin: item.coin,
                des: item.des.replaceAll('加速币', '金币'),
                record_type,
                created_at: item.created_at
            }
        })
        await postgres_knex.batchInsert('pay_record', list_batch, batch)
    }
    console.log('pay_record表数据迁移完成')
}
const copy_app = async () => {
    let list = await mysql_knex.raw(`
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
        from appStore
    `)
        .then(res => res[0])
    await postgres_knex("app").truncate()
    let batch = 100
    for (let i = 0; i < list.length; i += batch) {
        let list_batch = list.slice(i, i + batch).map(item => {
            return {
                app_id: item.app_id,
                country: item.country,
                name: item.name,
                origin_name: item.origin_name,
                bundle_id: item.bundle_id,
                des: item.des,
                icon: item.icon,
                platform: item.platform,
                price: item.price,
                genres: item.genres,
                single: item.single,
                created_at: item.created_at
            }
        })
        await postgres_knex.batchInsert('app', list_batch, batch)
    }
    console.log('app表数据迁移完成')
}
const copy_app_version = async () => {
    let list = await mysql_knex.raw(`
      select appid as app_id,
               country,
               version,
               des,
               download,
               official,
               file  as download_url,
               size,
               time  as created_at
        from versionStore
    `)
        .then(res => res[0])
    await postgres_knex("app_version").truncate()
    let batch = 100
    for (let i = 0; i < list.length; i += batch) {
        let list_batch = list.slice(i, i + batch).map(item => {
            return {
                app_id: item.app_id,
                country: item.country,
                version: item.version,
                des: item.des,
                download: item.download,
                official: item.official,
                download_url: item.download_url,
                size: item.size,
                created_at: item.created_at
            }
        })
        await postgres_knex.batchInsert('app_version', list_batch, batch)
    }
    console.log('app_version表数据迁移完成')
}
const copy_dump = async () => {
    let list = await mysql_knex.raw(`
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
            from dumpStore
    `)
        .then(res => res[0])
    await postgres_knex("dump").truncate()
    let batch = 100
    for (let i = 0; i < list.length; i += batch) {
        let list_batch = list.slice(i, i + batch).map(item => {
            return {
                app_id: item.app_id,
                country: item.country,
                version: item.version,
                name: item.name,
                icon: item.icon,
                link: item.link,
                bundle_id: item.bundle_id,
                size: item.size,
                price: item.price,
                status: item.status,
                created_at: item.created_at
            }
        })
        await postgres_knex.batchInsert('dump', list_batch, batch)
    }
    console.log('dump表数据迁移完成')
}
const copy_user_dump = async () => {
    let list = await mysql_knex.raw(`
           select userid as user_id,
                   appid  as app_id,
                   country,
                   version,
                   time   as created_at
            from userDump
    `)
        .then(res => res[0])
    await postgres_knex("user_dump").truncate()
    let batch = 100
    for (let i = 0; i < list.length; i += batch) {
        let list_batch = list.slice(i, i + batch).map(item => {
            return {
                user_id: item.user_id,
                app_id: item.app_id,
                country: item.country,
                version: item.version,
                created_at: item.created_at
            }
        })
        await postgres_knex.batchInsert('user_dump', list_batch, batch)
    }
}
const copy_user = async () => {
    let list = await mysql_knex.raw(`
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
        from userStore
    `)
        .then(res => res[0])
    await postgres_knex("user").truncate()
    let batch = 100
    for (let i = 0; i < list.length; i += batch) {
        let list_batch = list.slice(i, i + batch).map(item => {
            return {
                id: item.id,
                nick_name: item.nick_name,
                email: item.email,
                channel_code: item.channel_code,
                ip: item.ip,
                uid: item.uid,
                avatar: item.avatar,
                status: item.status,
                platform: item.platform,
                user_type: item.user_type,
                created_at: item.created_at
            }
        })
        await postgres_knex.batchInsert('user', list_batch, batch)
    }
    console.log('user表数据迁移完成')
}
const main = async () => {
    await copy_pay()
    await copy_pay_record()
    await copy_app()
    await copy_app_version()
    await copy_dump()
    await copy_user_dump()
    await copy_user()
    process.exit(0)
}
main()