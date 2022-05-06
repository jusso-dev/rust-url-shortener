use crate::lib::get_connection_pool;
use crate::ApiUrl;
use crate::models::UpdateUrl;
use diesel::prelude::*;
use chrono::{ NaiveDateTime, Utc };
use crate::models::{Url as DBUrl, NewUrl};

pub fn get_urls() -> Option<Vec<DBUrl>> {
    use crate::schema::urls::dsl::*;

    let connection = get_connection_pool().get().unwrap();
    
    let results = urls
        .load::<DBUrl>(&connection);
        
    match results {
        Ok(results) => Some(results),
        Err(_) => None
    }
}

pub fn create_url(url: ApiUrl) -> Option<bool> {
    use crate::schema::urls::dsl::*;

    let connection = get_connection_pool().get().unwrap();
    let new_url = NewUrl {
        short_url: &url.short_url,
        long_url: &url.long_url,
        created_at: datetime_helper()
    };
    
    let result = diesel::insert_into(urls)
            .values(&new_url)
            .execute(&connection);

    match result {
        Ok(_) => Some(true),
        Err(_) => Some(false)
    }
}

pub fn update_url(url: UpdateUrl) -> Option<bool> {
    println!("Updating user: {:?}", url);
    use crate::schema::urls::dsl::*;

    let connection = get_connection_pool().get().unwrap();
    let updated_url = UpdateUrl {
        id: url.id,
        short_url: url.short_url,
        long_url: url.long_url
    };

    let result = diesel::update(urls.find(url.id))
        .set(&updated_url)
        .execute(&connection);

    match result {
        Ok(_) => Some(true),
        Err(_) => Some(false)
    }
}

pub fn delete_user(id: i32) -> Option<bool> {
    use crate::schema::urls::dsl::*;

    let connection = get_connection_pool().get().unwrap();

    let result = diesel::delete(urls.find(id))
        .execute(&connection);

    match result {
        Ok(_) => Some(true),
        Err(_) => Some(false)
    }
}

fn datetime_helper() -> Option<NaiveDateTime> {
    let now = Utc::now().naive_utc();
    Some(now)
}