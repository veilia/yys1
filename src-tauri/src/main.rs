// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use yys1_lib::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
    // yys1_lib::run()
    ()
}
