#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket::request::{Form};
use rocket::response::{NamedFile, Redirect};
use rocket::http::{Cookies, Cookie};

use rocket_contrib::databases::diesel::{SqliteConnection};

use std::path::Path;
use diesel::prelude::*;

mod schema;
mod models;
use schema::users;
use models::{User, AdminUser};

#[database("sqlite_db")]
struct MyDatabase(SqliteConnection);

#[derive(FromForm, Debug)]
struct UserLogin {
    username: String,
    password: String,
}

fn get_user(conn: &diesel::SqliteConnection) -> Result<User,diesel::result::Error> {
    users::table.first::<User>(conn)
}

#[get("/")]
fn index(conn: MyDatabase) -> String {
    format!("{:?}", get_user(&conn)).to_string()
}

#[post("/login", data="<input>")]
fn new(input: Form<UserLogin>, mut cookies: Cookies) -> Redirect {
    let user_id = if input.username == "Kake" {
        "42"
    } else {
        "1337"
    };

    let cookie = Cookie::new("user_id", user_id);
    cookies.add_private(cookie);

    Redirect::to("/admin")
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/login")
}

#[get("/login")]
fn login() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/login.html")).ok()
}

#[get("/admin")]
fn admin_panel(admin: AdminUser) -> String {
    format!("Welcome in, administrator: {:?}", admin)
}

#[get("/admin", rank=2)]
fn admin_panel_user(user: User) -> String {
    format!("Sorry, admins only... {:?}", user)
}

#[get("/admin", rank=3)]
fn admin_panel_redirect() -> Redirect {
    Redirect::to("/login")
}

fn main() {
    rocket::ignite()
        .attach(MyDatabase::fairing())
        .mount("/", routes![index, new, login, admin_panel, admin_panel_user, admin_panel_redirect, logout])
        .launch();
}
