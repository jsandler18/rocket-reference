use chrono::naive::NaiveDate;
use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;
use self::schema::users;

mod schema {
    table! {
        users (id) {
            id -> Int4,
            firstname -> Varchar,
            lastname -> Varchar,
            birthday -> Date,
            username -> Varchar,
            password -> Varchar,
        }
    }
}


#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub birthday: NaiveDate,
    pub username: String,
    pub password: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub birthday: NaiveDate,
    pub username: String,
    pub password: String
}

#[derive(FromForm)]
pub struct UserForm {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
    pub birthday: String
}

impl User {
    pub fn all(conn: &PgConnection) -> Vec<User> {
        users::table.load::<User>(conn).expect("Error loading users")
    }

    pub fn lookup(id: i32, conn: &PgConnection) -> User {
        users::table.find(id).get_result::<User>(&*conn).expect("Error loading users")
    }

    pub fn insert(new_user: NewUser, conn: &PgConnection) -> bool {
        !diesel::insert_into(users::table).values(&new_user).execute(conn).is_err()
    }
}
