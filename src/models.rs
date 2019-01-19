use rocket::request::{Request, FromRequest, Outcome};

use rocket_contrib::databases::diesel::{Queryable};
use rocket::outcome::IntoOutcome;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub privileged: bool,
}

#[derive(Debug)]
pub struct AdminUser {
   pub user: User,
}

impl <'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<User, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id: i32| { if id == 42 {Some(User{id, username:"kake".to_string(), password:"hest".to_string(), privileged: false})} else { None }})
            .or_forward(())
    }
}

impl <'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AdminUser, ()> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id: usize| { if id == 1337 {Some(AdminUser{user:User{id:0, username:"Kake".to_string(), password:"Secret".to_string(), privileged:true}})} else { None}})
                .or_forward(())
    }
}
