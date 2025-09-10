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
