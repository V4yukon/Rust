#[macro_use]

extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema {
    table! {
        users (id) {
            id -> Int4,
            name -> Varchar,
        }
    }
}

use schema::users;


#[derive(Queryable)]

struct User{
    id: i32,
    name: String,
}

fn get_users(conn: &PgConnection) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;
    user.load::<User>(conn)
}

fn main() {
    let database_url = "postgres://localhost/mydata";
    let connection = PgConnection::establish(&database_url).
        expect(&format!("Error connecting to {}", database_url));
    let users = get_users(&connection).expect("Error loading user");

    for user in users{
        println!("{}: {}", user.id, user.name);`
}






