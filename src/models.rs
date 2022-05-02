use chrono::{ NaiveDateTime };
use serde::{Serialize};

#[derive(Serialize, Debug, Queryable)]
pub struct Url {
    pub id: i32,
    pub short_url: String,
    pub long_url: String,
    pub created_at: NaiveDateTime
}