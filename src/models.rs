use argonautica::{Hasher, Verifier};
use rocket::request::{self, FromRequest, Request};

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::outcome::{IntoOutcome, Outcome};
use rocket_contrib::databases::diesel::{Insertable, Queryable, QueryableByName, SqliteConnection};

use crate::schema::users;

#[database("sqlite_db")]
pub struct Database(SqliteConnection);

#[derive(Queryable, Debug, PartialEq, QueryableByName)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub privileged: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub privileged: bool,
}

#[derive(Debug)]
pub struct Admin {
    pub user: User,
}

const HASHKEY: &'static str = "ARSTARST";

impl Database {
    pub fn get_user(&self, id: i32) -> Result<User, Error> {
        use crate::schema::users::dsl::users;

        users.find(id).first::<User>(&self.0)
    }

    pub fn create_user(
        &self,
        username: &str,
        password: &str,
        privileged: bool,
    ) -> Result<usize, Error> {
        use crate::schema::users::dsl::users;

        let hash = Hasher::default().with_secret_key(HASHKEY).with_password(password).hash();
        if let Ok(hash) = hash {
            diesel::insert_into(users)
                .values(NewUser {
                    username: username.to_string(),
                    password: hash,
                    privileged,
                })
                .execute(&self.0)
        } else {
            Err(Error::__Nonexhaustive)
        }
    }

    pub fn login_user(&self, username_prov: &str, password_prov: &str) -> Result<User, Error> {
        use crate::schema::users::dsl::*;

        let user: User = users.filter(username.eq(username_prov)).first(&self.0)?;
        let verified = Verifier::default()
            .with_secret_key(HASHKEY)
            .with_hash(&user.password)
            .with_password(password_prov)
            .verify();

        match verified {
            Ok(true) => Ok(user),
            Ok(false) => Err(Error::__Nonexhaustive),
            Err(_) => Err(Error::__Nonexhaustive),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let db = request.guard::<Database>()?;
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .and_then(|id| db.get_user(id).ok())
            .or_forward(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
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
