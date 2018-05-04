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
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String
}

#[derive(FromForm)]
pub struct UserForm {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
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

impl From<UserForm> for NewUser {
    fn from(form: UserForm) -> Self {
        NewUser {
            username: form.username,
            first_name: form.first_name,
            last_name: form.last_name,
            password_hash: sha256_crypt::hash(form.password.as_str()).expect("Could Not has password")
        }
    }
}
