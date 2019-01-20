use rocket::request::{self, Request, FromRequest};

use rocket_contrib::databases::diesel::{Queryable, SqliteConnection};
use rocket::outcome::{IntoOutcome, Outcome};
use diesel::{RunQueryDsl, QueryDsl};
use diesel::result::Error;

use crate::schema::users::dsl::users;

#[database("sqlite_db")]
pub struct Database(SqliteConnection);

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub privileged: bool,
}

#[derive(Debug)]
pub struct Admin{
   pub user: User,
}

impl Database {
    pub fn get_user(&self, id: i32) -> Result<User, Error> {
        users.find(id).first::<User>(&self.0)
    }
}

impl <'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let db = request.guard::<Database>()?;
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id| db.get_user(id).ok())
            .or_forward(())
    }
}

impl <'a, 'r> FromRequest<'a, 'r> for Admin{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, ()> {
        let user = request.guard::<User>()?;

        if user.privileged {
            Outcome::Success(Admin { user })
        } else {
            Outcome::Forward(())
        }
    }
}
