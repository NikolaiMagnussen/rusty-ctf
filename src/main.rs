#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket::request::{Form};
use rocket::response::{NamedFile, Redirect};
use rocket::http::{Cookies, Cookie};


use std::path::Path;

mod schema;
mod models;
use models::{User, Admin, Database};

#[derive(FromForm, Debug)]
struct UserLogin {
    username: String,
    password: String,
}

#[get("/")]
fn index() -> &'static str {
    "Heisann da"
}

#[post("/login", data="<input>")]
fn new(input: Form<UserLogin>, mut cookies: Cookies) -> Redirect {
    let user_id = if input.username == "user" {
        "0"
    } else if input.username == "admin" {
        "1"
    } else {
        "-1"
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
fn admin_panel(admin: Admin) -> String {
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
        .attach(Database::fairing())
        .mount("/", routes![index, new, login, admin_panel, admin_panel_user, admin_panel_redirect, logout])
        .launch();
}
