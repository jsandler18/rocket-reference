use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;
use self::schema::users;
use pwhash::sha256_crypt;

mod schema {
    table! {
        users (id) {
            id -> Int4,
            username -> Text,
            first_name -> Text,
            last_name -> Text,
            password_hash -> Varchar,
        }
    }
}


#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
    username: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    password_hash: &'a str
}

impl User {

    pub fn all(conn: &PgConnection) -> Vec<User> {
        users::table.load::<User>(conn).expect("Error loading users")
    }

    pub fn lookup(id: i32, conn: &PgConnection) -> User {
        users::table.find(id).get_result::<User>(&*conn).expect("Error loading users")
    }

    pub fn insert(username: &str, password: &str, first_name: &str, last_name: &str, conn: &PgConnection) -> Option<User> {
        let pwhash = sha256_crypt::hash(password).expect("Could Not Hash Password");
        let new_user = NewUser { 
            username, 
            first_name, 
            last_name, 
            password_hash: pwhash.as_str()
        };
        diesel::insert_into(users::table).values(&new_user).get_result::<User>(conn).ok()
    }

    pub fn authenticate(user: &str, pass: &str, conn: &PgConnection) -> Option<i32> {
        use self::schema::users::dsl::*;

        users.filter(username.eq(user)).get_result::<User>(conn).ok()
            .and_then(|u| if sha256_crypt::verify(pass,u.password_hash.as_str()) {
                                Some(u.id)
                            } else {
                                None
                            })
    }
}
