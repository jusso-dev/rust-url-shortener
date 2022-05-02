use crate::lib::establish_connection;
use crate::models::{Url};
use diesel::prelude::*;

pub fn get_urls() -> Vec<Url> {
    use crate::schema::urls::dsl::*;

    let connection = establish_connection();
    let results = urls
        .load::<Url>(&connection)
        .unwrap();
    
    return results;
}