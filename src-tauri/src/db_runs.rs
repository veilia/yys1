use sqlx::{Pool, QueryBuilder, Sqlite, query};
use uuid::Uuid;

use crate::structs::{Act, RunRec};

pub async fn ins_run(
    act: Act,
    cost: u32,
    recs: Vec<RunRec>,
    pool: &Pool<Sqlite>,
) -> Result<(), sqlx::Error> {
    // let conn = AnyConnection::connect("sqlite://yys.db").await.unwrap();
    let run_id = Uuid::now_v7().to_string();
    let sql = r#"
        insert into runs (id, cost, aid)
        values (?, ?, ?)
    "#;

    query(sql)
        .bind(&run_id)
        .bind(cost)
        .bind(act.id)
        .execute(pool)
        .await?;
    if !recs.is_empty() {
        let mut query_builder: QueryBuilder<Sqlite> =
            QueryBuilder::new("INSERT INTO run_drops (id, run_id, rid, num) ");

        query_builder.push_values(recs, |mut b, rec| {
            b.push_bind(Uuid::now_v7().to_string())
                .push_bind(&run_id)
                .push_bind(rec.id.clone())
                .push_bind(rec.num);
        });

        let query = query_builder.build();
        query.execute(pool).await?;
    }

    Ok(())
}