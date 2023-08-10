use crate::{db, error_handler::AppError};
use crate::schema::users;
use chrono::DateTime;
use chrono::offset::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    // pub email_verified_at: Option<DateTime<Utc>>,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn all() -> Result<Vec<Self>, anyhow::Error> {
        let mut conn = db::connection()?;
        let users = users::table.load::<User>(&mut conn)?;
        Ok(users)
    }

    pub fn find(id: i32) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let user = users::table.filter(users::id.eq(id)).first(&mut conn)?;
        Ok(user)
    }

    pub fn email(email: &str) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let user = users::table.filter(users::email.eq(email)).first(&mut conn)?;
        Ok(user)
    }

    pub fn create(user: CreateUser) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let user = CreateUser::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut conn)?;
        Ok(user)
    }

    pub fn update(id: i32, user: CreateUser) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(&mut conn)?;
        Ok(user)
    }

    pub fn delete(id: i32) -> Result<usize, anyhow::Error> {
        let mut conn = db::connection()?;
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl CreateUser {
    fn from(user: CreateUser) -> CreateUser {
        CreateUser {
            name: user.name,
            email: user.email,
            // email_verified_at: user.email_verified_at,
            // created_at: user.created_at,
            // updated_at: user.updated_at
        }
    }
}

