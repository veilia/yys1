use std::{str::FromStr, time::Duration};

use sqlx::{
    Pool, Sqlite, migrate,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use structs::Act;

use crate::{
    db_act::{del_act, ins_act, sel_act, sel_all_act},
    db_rec::{del_rec, ins_rec, sel_rec},
    db_runs::ins_run,
    structs::RunRec,
};

mod db_act;
mod db_rec;
mod db_runs;
mod db_usr;
mod structs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect_with(
            SqliteConnectOptions::from_str("sqlite://yys.db")?
                .create_if_missing(true)
                .busy_timeout(Duration::from_secs(1)),
        )
        .await;

    let pool = match pool {
        Ok(pool) => {
            // create_table(&pool).await?;
            migrate!("./migrations")
                .run(&pool)
                .await
                .inspect(|_| println!("Database migration completed"))
                .expect("Failed to migrate database");
            pool
        }
        Err(e) => {
            eprintln!("Error connecting to database: {e}");
            return Err(e);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            greet,
            init_db,
            add_act,
            get_all_act,
            get_acts,
            rm_act,
            add_rec,
            get_rec,
            rm_rec,
            add_run
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn init_db(pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let res = db_usr::init_db(&pool).await;
    match res {
        Ok(_) => Ok("init success".to_string()),
        Err(e) => Err(format!("Error init db: {e}")),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn add_act(mut act: Act, pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let tag = match act.tag {
        Some(t) => {
            if t.is_empty() {
                Some("日常".to_string())
            } else {
                Some(t)
            }
        }
        _ => Some("日常".to_string()),
    };

    act.tag = tag;

    let res = ins_act(&pool, act).await;
    match res {
        Ok(_) => Ok("Activity add success".to_string()),
        Err(_) => Err(format!("Error adding activity")),
    }
}

#[tauri::command]
async fn get_all_act(pool: tauri::State<'_, Pool<Sqlite>>) -> Result<Vec<Act>, String> {
    let res = sel_all_act(&pool).await;
    match res {
        Ok(a) => Ok(a),
        Err(e) => Err(format!("Error get activity: {e}")),
    }
}

#[tauri::command]
async fn get_acts(
    tag: Option<String>,
    pool: tauri::State<'_, Pool<Sqlite>>,
) -> Result<Vec<Act>, String> {
    let tag = match tag {
        Some(t) => {
            if t.is_empty() {
                "日常副本".to_string()
            } else {
                t
            }
        }
        None => "日常副本".to_string(),
    };

    let res = sel_act(&pool, tag).await;
    match res {
        Ok(a) => Ok(a),
        Err(e) => Err(format!("Error get activity: {e}")),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn rm_act(id: &str, pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let res = del_act(&pool, id.to_string()).await;
    match res {
        Ok(_) => Ok("Activity rm success".to_string()),
        Err(_) => Err(format!("Error remove activity")),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn add_rec(name: &str, pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let res = ins_rec(&pool, name).await;
    match res {
        Ok(_) => Ok("Resource type added successfully".to_string()),
        Err(_) => Err(format!("Error adding resource type")),
    }
}

#[tauri::command]
async fn get_rec(pool: tauri::State<'_, Pool<Sqlite>>) -> Result<Vec<structs::Rec>, String> {
    let res = sel_rec(&pool).await;
    match res {
        Ok(a) => Ok(a),
        Err(_) => Err(format!("Error get resource type")),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn rm_rec(id: &str, pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let res = del_rec(&pool, id.to_string()).await;
    match res {
        Ok(_) => Ok("Resource type rm successfully".to_string()),
        Err(_) => Err(format!("Error remove resource type")),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn add_run(
    act: &str,
    cost: u32,
    recs: Vec<RunRec>,
    pool: tauri::State<'_, Pool<Sqlite>>,
) -> Result<String, String> {
    let recs = recs
        .into_iter()
        .filter(|r| !r.id.clone().unwrap().trim().is_empty() && r.num.unwrap() > 0)
        .collect::<Vec<RunRec>>();

    let res = ins_run(
        Act {
            id: Some(act.to_string()),
            ..Default::default()
        },
        cost,
        recs,
        &pool,
    )
    .await;
    match res {
        Ok(_) => Ok("Add run success".to_string()),
        Err(_) => Err(format!("Error add run")),
    }
}
