use crate::schema::urls;

use chrono::{ NaiveDateTime };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Debug, AsChangeset)]
pub struct Url {
    pub id: i32,
    pub short_url: String,
    pub long_url: String,
    pub created_at: NaiveDateTime
}

#[derive(Eq, PartialEq, Insertable, Deserialize)]
#[table_name = "urls"]
pub struct NewUrl<'a> {
    pub short_url: &'a str,
    pub long_url: &'a str,
    pub created_at: Option<NaiveDateTime>
}

// Used to create new URLs from API endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUrl {
    pub short_url: String,
    pub long_url: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "urls"]
pub struct UpdateUrl {
    pub id: i32,
    pub short_url: String,
    pub long_url: String
}

#[derive(Debug, Serialize)]
pub struct HealthCheck {
    pub message: String
}

#[derive(Debug)]
pub enum Validation {
    Success,
    UrlInvalid,
    UrlDuplicate
}