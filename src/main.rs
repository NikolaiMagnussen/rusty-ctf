#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{NamedFile, Redirect};

use std::path::Path;

mod models;
mod schema;
use models::{Admin, Database, User};

#[derive(FromForm, Debug)]
struct UserLogin {
    username: String,
    password: String,
}

#[derive(FromForm, Debug)]
struct UserCreation {
    username: String,
    password: String,
    privileged: bool,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/login")
}

#[post("/login", data = "<input>")]
fn new(input: Form<UserLogin>, mut cookies: Cookies, conn: Database) -> Redirect {
    if let Ok(user) = conn.login_user(&input.username, &input.password) {
        let cookie = Cookie::new("user_id", user.id.to_string());
        cookies.add_private(cookie);
    }

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

#[get("/register")]
fn register() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/register.html")).ok()
}

#[post("/register", data = "<input>")]
fn register_user(input: Form<UserCreation>, conn: Database) -> Redirect {
    if conn
        .create_user(&input.username, &input.password, input.privileged)
        .is_ok()
    {
        println!("Successfully created new user");
    } else {
        println!("Error creating new user");
    }

    Redirect::to("/login")
}

#[get("/admin")]
fn admin_panel(admin: Admin) -> String {
    format!("Welcome in, administrator: {:?}", admin)
}

#[get("/admin", rank = 2)]
fn admin_panel_user(user: User) -> String {
    format!("Sorry, admins only... {:?}", user)
}

#[get("/admin", rank = 3)]
fn admin_panel_redirect() -> Redirect {
    Redirect::to("/login")
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .mount(
            "/",
            routes![
                index,
                new,
                login,
                admin_panel,
                admin_panel_user,
                admin_panel_redirect,
                logout,
                register,
                register_user
            ],
        )
        .launch();
}
