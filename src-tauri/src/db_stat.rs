use sqlx::{Pool, Sqlite};

use crate::structs::{ResAct, ResRec};

pub async fn sel_act_stats(act_id: &str, pool: &Pool<Sqlite>) -> Result<ResAct, sqlx::Error> {
    // 执行查询
    let res: Option<ResAct> = sqlx::query_as!(
        ResAct,
        r#"
            SELECT
                count(*) as cnt,
                sum(cost) as t_cost
            FROM
                runs
            WHERE aid=?
        "#,
        act_id
    )
    .fetch_optional(pool)
    .await?;

    match res {
        Some(r) => Ok(r),
        _ => Ok(ResAct {
            cnt: Some(0),
            t_cost: Some(0),
        }),
    }
}

pub async fn sel_act_recs(act_id: &str, pool: &Pool<Sqlite>) -> Result<Vec<ResRec>, sqlx::Error> {
    // 执行查询
    let res: Vec<ResRec> = sqlx::query_as!(
        ResRec,
        r#"
            SELECT
                rd.rid as id,
                sum(rd.num) as num
            FROM
                runs r
            INNER JOIN
                run_drops rd ON rd.run_id=r.id
            WHERE
                aid=?
            GROUP BY
                rd.rid
        "#,
        act_id
    )
    .fetch_all(pool)
    .await?;

    Ok(res)
}
