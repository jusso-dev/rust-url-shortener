use crate::ApiUrl;
use crate::models::UpdateUrl;
use chrono::Utc;
use diesel::prelude::*;
use chrono::{ NaiveDateTime };

use crate::lib::establish_connection;
use crate::models::{Url as DBUrl, NewUrl};

pub fn get_urls() -> Vec<DBUrl> {
    use crate::schema::urls::dsl::*;

    let connection = establish_connection();
    let results = urls
        .load::<DBUrl>(&connection)
        .unwrap();
    
    return results;
}

pub fn create_url(url: ApiUrl) {
    use crate::schema::urls::dsl::*;

    let connection = establish_connection();
    let new_url = NewUrl {
        short_url: &url.short_url,
        long_url: &url.long_url,
        created_at: datetime_helper()
    };

    diesel::insert_into(urls)
        .values(&new_url)
        .execute(&connection)
        .expect("Error saving new url.");
}

pub fn update_url(url: UpdateUrl) {
    println!("Updating user: {:?}", url);
    use crate::schema::urls::dsl::*;

    let connection = establish_connection();
    let existing_record = urls.find(url.id).first::<DBUrl>(&connection).expect("Error loading url");

    diesel::update(urls.find(url.id))
        .set(&existing_record)
        .execute(&connection)
        .expect("Error updating user");
}

pub fn delete_user(id: i32) {
    use crate::schema::urls::dsl::*;

    let connection = establish_connection();
    diesel::delete(urls.find(id))
        .execute(&connection)
        .expect("Error deleting user");
}

fn datetime_helper() -> Option<NaiveDateTime> {
    let now = Utc::now().naive_utc();
    Some(now)
}