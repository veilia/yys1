use sqlx::{Pool, Sqlite, query, query_as};
use uuid::Uuid;

use crate::structs::Act;

pub async fn ins_act(pool: &Pool<Sqlite>, act: Act) -> Result<(), sqlx::Error> {
    // let conn = AnyConnection::connect("sqlite://yys.db").await.unwrap();
    let sql = r#"
        insert into act (id, name, cost, tag)
        values (?, ?, ?, ?)
    "#;

    query(sql)
        .bind(Uuid::now_v7().to_string())
        .bind(act.name)
        .bind(act.cost)
        .bind(act.tag)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn sel_all_act(pool: &Pool<Sqlite>) -> Result<Vec<Act>, sqlx::Error> {
    let sql = r#"
        select * from act
    "#;

    let activities = query_as::<_, Act>(sql).fetch_all(pool).await?;

    Ok(activities)
}

pub async fn sel_act(pool: &Pool<Sqlite>, tag: String) -> Result<Vec<Act>, sqlx::Error> {
    let sql = r#"
        select * from act where tag = ?
    "#;

    let activities = query_as::<_, Act>(sql).bind(tag).fetch_all(pool).await?;

    Ok(activities)
}

pub async fn del_act(pool: &Pool<Sqlite>, id: String) -> Result<(), sqlx::Error> {
    let sql = r#"
        delete from act where id = ?
    "#;

    query(sql).bind(id).execute(pool).await?;

    Ok(())
}
