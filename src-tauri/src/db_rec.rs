use sqlx::{Pool, Sqlite, query, query_as};
use uuid::Uuid;

use crate::structs::Rec;

pub async fn ins_rec(pool: &Pool<Sqlite>, name: &str) -> Result<(), sqlx::Error> {
    let sql = r#"
        insert into rec (id, name)
        values (?, ?)
    "#;

    let res = query(sql)
        .bind(Uuid::now_v7().to_string())
        .bind(name)
        .execute(pool)
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn sel_rec(pool: &Pool<Sqlite>) -> Result<Vec<Rec>, sqlx::Error> {
    let sql = r#"
        select * from rec
    "#;

    let types = query_as::<_, Rec>(sql).fetch_all(pool).await?;

    Ok(types)
}

pub async fn del_rec(pool: &Pool<Sqlite>, id: String) -> Result<(), sqlx::Error> {
    let sql = r#"
        delete from rec where id = ?
    "#;

    query(sql).bind(id).execute(pool).await?;

    Ok(())
}
