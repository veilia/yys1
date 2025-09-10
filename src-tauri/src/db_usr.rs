use sqlx::{Pool, Sqlite, query};
use uuid::Uuid;

pub async fn init_db(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let sql1 = r#"
        DELETE FROM rec
    "#;

    let res1 = query(sql1).execute(pool).await;
    match res1 {
        Err(e) => return Err(e),
        _ => {}
    };

    let sql1 = r#"
        DELETE FROM act
    "#;

    let res1 = query(sql1).execute(pool).await;
    match res1 {
        Err(e) => return Err(e),
        _ => {}
    };

    let sql1 = r#"
        DELETE FROM runs
    "#;

    let res1 = query(sql1).execute(pool).await;
    match res1 {
        Err(e) => return Err(e),
        _ => {}
    };

    let sql1 = r#"
        DELETE FROM run_drops
    "#;

    let res1 = query(sql1).execute(pool).await;
    match res1 {
        Err(e) => return Err(e),
        _ => {}
    };

    let sql2 = r#"
        insert into rec (id, name)
        values
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?),
            (?, ?);
    "#;

    let sql3 = r#"
        insert into act (id, name, cost)
        values
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?),
            (?, ?, ?);
    "#;

    let res2 = query(sql2)
        .bind(Uuid::now_v7().to_string())
        .bind("勾玉")
        .bind(Uuid::now_v7().to_string())
        .bind("体力")
        .bind(Uuid::now_v7().to_string())
        .bind("蓝票")
        .bind(Uuid::now_v7().to_string())
        .bind("金币")
        .bind(Uuid::now_v7().to_string())
        .bind("2星白蛋")
        .bind(Uuid::now_v7().to_string())
        .bind("3星白蛋")
        .bind(Uuid::now_v7().to_string())
        .bind("4星白蛋")
        .bind(Uuid::now_v7().to_string())
        .bind("5星白蛋")
        .bind(Uuid::now_v7().to_string())
        .bind("蓝蛋")
        .bind(Uuid::now_v7().to_string())
        .bind("n卡")
        .bind(Uuid::now_v7().to_string())
        .bind("1-3星御魂")
        .bind(Uuid::now_v7().to_string())
        .bind("4星御魂")
        .bind(Uuid::now_v7().to_string())
        .bind("5星御魂")
        .bind(Uuid::now_v7().to_string())
        .bind("6星御魂")
        .bind(Uuid::now_v7().to_string())
        .bind("黑蛋")
        .bind(Uuid::now_v7().to_string())
        .bind("随机SSR")
        .bind(Uuid::now_v7().to_string())
        .bind("1星变异卡")
        .bind(Uuid::now_v7().to_string())
        .bind("2星变异卡")
        .bind(Uuid::now_v7().to_string())
        .bind("3星变异卡")
        .bind(Uuid::now_v7().to_string())
        .bind("4星变异卡")
        .bind(Uuid::now_v7().to_string())
        .bind("5星变异卡")
        .bind(Uuid::now_v7().to_string())
        .bind("6星变异卡")
        .bind(Uuid::now_v7().to_string())
        .bind("1星太阴")
        .bind(Uuid::now_v7().to_string())
        .bind("2星太阴")
        .bind(Uuid::now_v7().to_string())
        .bind("3星太阴")
        .bind(Uuid::now_v7().to_string())
        .bind("4星太阴")
        .bind(Uuid::now_v7().to_string())
        .bind("5星太阴")
        .bind(Uuid::now_v7().to_string())
        .bind("6星太阴")
        .bind(Uuid::now_v7().to_string())
        .bind("4星太鼓")
        .bind(Uuid::now_v7().to_string())
        .bind("5星太鼓")
        .bind(Uuid::now_v7().to_string())
        .bind("6星太鼓")
        .bind(Uuid::now_v7().to_string())
        .bind("4星斗鱼")
        .bind(Uuid::now_v7().to_string())
        .bind("5星斗鱼")
        .bind(Uuid::now_v7().to_string())
        .bind("6星斗鱼")
        .bind(Uuid::now_v7().to_string())
        .bind("紫蛇皮")
        .bind(Uuid::now_v7().to_string())
        .bind("金蛇皮")
        .bind(Uuid::now_v7().to_string())
        .bind("逢魔皮")
        .bind(Uuid::now_v7().to_string())
        .bind("绘本")
        .bind(Uuid::now_v7().to_string())
        .bind("百鬼票")
        .bind(Uuid::now_v7().to_string())
        .bind("御灵票")
        .bind(Uuid::now_v7().to_string())
        .bind("活动币")
        .execute(pool)
        .await;

    let res3 = query(sql3)
        .bind(Uuid::now_v7().to_string())
        .bind("魂十")
        .bind(4)
        .bind(Uuid::now_v7().to_string())
        .bind("悲鸣")
        .bind(8)
        .bind(Uuid::now_v7().to_string())
        .bind("神罚")
        .bind(21)
        .bind(Uuid::now_v7().to_string())
        .bind("困28")
        .bind(3)
        .bind(Uuid::now_v7().to_string())
        .bind("结界突破")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("地域鬼王")
        .bind(1)
        .bind(Uuid::now_v7().to_string())
        .bind("道馆")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("狭间暗域")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("逢魔")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("逢魔首领")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("麒麟")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("斗技")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("经验妖怪")
        .bind(12)
        .bind(Uuid::now_v7().to_string())
        .bind("金币妖怪")
        .bind(12)
        .bind(Uuid::now_v7().to_string())
        .bind("结界卡")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("结界寄养")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("悬赏封印")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("日常奖励")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("花合战")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("成就")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("限时小活动")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("平安奇谭")
        .bind(0)
        .bind(Uuid::now_v7().to_string())
        .bind("真蛇")
        .bind(60)
        .bind(Uuid::now_v7().to_string())
        .bind("永生之海")
        .bind(24)
        .bind(Uuid::now_v7().to_string())
        .bind("契灵")
        .bind(6)
        .execute(pool)
        .await;

    match res2 {
        Ok(_) => match res3 {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
