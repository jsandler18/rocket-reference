
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate rocket_contrib;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate tera;
extern crate pwhash;

pub mod user;
pub mod db;

use rocket_contrib::Template;
use rocket::response::{NamedFile, Redirect};
use rocket::request::Form;
use tera::Context;


#[get("/user/<id>")]
fn user(id: i32, conn: db::DbConn) -> Template {
    let user = user::User::lookup(id, &conn);
    Template::render("user", &user)
}

#[get("/")]
fn index(conn: db::DbConn) -> Template {
    let results = user::User::all(&conn);
    let mut res = Context::new();
    res.insert("users", &results);
    Template::render("index", &res)
}

#[get("/create")]
fn create_user_page() -> NamedFile {
    NamedFile::open("./static/create.html").expect("could not find file")
}

#[post("/create", data="<user>")]
fn create_user(user: Form<user::UserForm>, conn: db::DbConn) -> Redirect {
    let user = user.into_inner();
    user::User::insert(user.into(), &conn);
    Redirect::to("/")
}

fn main() {
    rocket::ignite().mount("/", routes![index, user, create_user_page, create_user]).manage(db::init_pool()).attach(Template::fairing()).launch();
}
