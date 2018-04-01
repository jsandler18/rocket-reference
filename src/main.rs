
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate rocket_contrib;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate tera;

pub mod user;
pub mod db;

use rocket_contrib::Template;
use tera::Context;


#[get("/<id>")]
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

fn main() {
    rocket::ignite().mount("/", routes![index, user]).manage(db::init_pool()).attach(Template::fairing()).launch();
}
