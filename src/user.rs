use chrono::naive::NaiveDate;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use self::schema::users::dsl::{users};

mod schema {
    table! {
        users (id) {
            id -> Int4,
            firstname -> Varchar,
            lastname -> Varchar,
            birthday -> Date,
        }
    }
}


#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate
}

impl User {
    pub fn all(conn: &PgConnection) -> Vec<User> {
        users.load::<User>(conn).expect("Error loading users")
    }

    pub fn lookup(id: i32, conn: &PgConnection) -> User {
        users.find(id).get_result::<User>(&*conn).expect("Error loading users")
    }
}
