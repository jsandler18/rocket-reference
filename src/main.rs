
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
use rocket::response::{NamedFile, Redirect, Flash};
use rocket::http::{Cookies, Cookie};
use rocket::request::{Form, FlashMessage};
use tera::Context;


#[get("/user/<id>")]
fn user(id: i32, mut cookies: Cookies, conn: db::DbConn) -> Template {
    let user = user::User::lookup(id, &conn);
    let mut ctx = Context::new();
    ctx.insert("user", &user);
    println!("{}",cookies.get_private("ID").is_some());
    ctx.insert("is_you", &(cookies.get_private("ID").map_or(false, |i| i.value() == id.to_string())));
    Template::render("user", &ctx)
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

#[get("/login")]
fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut ctx = Context::new();
    match flash {
        None => ctx.insert("error", &false),
        Some(msg) => {
            ctx.insert("error", &true);
            ctx.insert("error_name", msg.name());
            ctx.insert("error_msg", msg.msg());
        }
    }
    Template::render("login",&ctx)
}

#[post("/login", data="<form>")]
fn login(mut cookies: Cookies, form: Form<user::LoginForm>, conn: db::DbConn) -> Result<Redirect, Flash<Redirect>> {
   let form = form.into_inner();
   match user::User::authenticate(form.username.as_str(), form.password.as_str(), &conn) {
        None => Err(Flash::error(Redirect::to("/login"), "Invalid username or password")),
        Some(id) => {
            let auth_cookie = Cookie::build("ID", id.to_string()).secure(false).http_only(true).finish();
            cookies.add_private(auth_cookie);
            Ok(Redirect::to(&format!("/user/{}",id)))
        }
   } 
}

fn main() {
    rocket::ignite().mount("/", routes![index, user, create_user_page, create_user, login_page, login]).manage(db::init_pool()).attach(Template::fairing()).launch();
}
