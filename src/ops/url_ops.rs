use crate::{lib::get_connection_pool};
use crate::ApiUrl;
use crate::models::{UpdateUrl, Validation};
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

pub fn get_url(url:String) -> Option<DBUrl> {

    use crate::schema::urls::dsl::*;
    let connection = get_connection_pool().get().unwrap();
    
    // Return a single url
    let result:DBUrl = urls
        .filter(short_url.eq(url))
        .first(&connection)
        .unwrap();
        
    if result.short_url == "" {
        None
    } else {
        Some(result)
    }
}

pub async fn create_url(url: ApiUrl) -> Option<Validation> {

    if is_duplicate_url(url.long_url.clone()) {
        print!("Duplicate URL");
        return Some(Validation::UrlDuplicate);
    }

    if !validate_url(url.long_url.clone()).await {
        print!("Invalid URL");
        return Some(Validation::UrlInvalid);
    }

    use crate::schema::urls::dsl::*;

    let connection = get_connection_pool().get().unwrap();
    let new_url = NewUrl {
        short_url: &url.short_url,
        long_url: &url.long_url,
        created_at: datetime_helper()
    };
    
    let result = diesel::insert_into(urls)
            .values(&new_url)
            .execute(&connection)
            .unwrap();

    match result {
        1 => Some(Validation::Success),
        _ => None
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

pub fn delete_user(_id: i32) -> Option<bool> {
    use crate::schema::urls::dsl::*;

    let connection = get_connection_pool().get().unwrap();

    let result = diesel::delete(urls.find(_id))
        .execute(&connection);

    match result {
        Ok(_) => Some(true),
        Err(_) => Some(false)
    }
}

async fn validate_url(check_long_url:String) -> bool {

    let resp = reqwest::get(check_long_url)
        .await
        .unwrap();

    if resp.status().is_success() {
        return true;
    } else {
        println!("Invalid URL");
        return false;
    }
}

// Check database for duplicate short_url
fn is_duplicate_url(check_long_url:String) -> bool {

    if check_long_url == "" {
        return false;
    }

    use crate::schema::urls::dsl::*;
    let connection = get_connection_pool().get().unwrap();
    
    let url_not_found = urls
    .filter(long_url.eq(&check_long_url))
    .first::<DBUrl>(&connection)
    .is_err();

    match url_not_found {
        true => false,
        false => true
    }
}

fn datetime_helper() -> Option<NaiveDateTime> {
    let now = Utc::now().naive_utc();
    Some(now)
}