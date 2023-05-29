#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod schema;
use self::schema::users;

#[derive(Queryable)]
struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::NaiveDateTime,
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

    let results = users::table.limit(5).load::<User>(&connection).expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} ({}) - {}", user.name, user.email, user.created_at);
    }
}
