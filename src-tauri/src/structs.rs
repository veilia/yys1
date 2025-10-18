use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct Rec {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct RunRec {
    pub id: Option<String>,
    pub num: Option<u32>,
}

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct Act {
    pub id: Option<String>,
    pub name: Option<String>,
    pub cost: Option<u32>,
    pub tag: Option<String>,
}

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct ResAct {
    pub cnt: Option<i64>,
    pub t_cost: Option<i64>,
}

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct ResRec {
    pub id: Option<String>,
    pub num: Option<i64>,
}

#[derive(Default, Serialize, Deserialize, FromRow)]
pub struct ResStat {
    pub name: Option<String>,
    pub num: Option<i64>,
}