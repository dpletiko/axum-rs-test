use crate::users::{User, Users};
use crate::utils::Mailable;
use crate::{db, auth::routes::{AuthRequest, AuthCodeRequest, AuthResponse}, utils::Mailer};
use crate::schema::{users, auth};
use chrono::{DateTime, Duration};
use chrono::offset::Utc;
use diesel::prelude::*;
use diesel::Queryable;
use mail_send::mail_builder::headers::address::Address;
use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};
use async_trait::async_trait;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = auth)]
// #[changeset_options(treat_none_as_null = "true")]
pub struct Auth {
    pub user_id: i32,
    pub pin: String,
    pub tries: i32,
    pub expires_at: DateTime<Utc>,
    pub locked_until: Option<DateTime<Utc>>,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Queryable, PartialEq, Insertable, Identifiable, Associations, AsChangeset)]
#[diesel(belongs_to(Users, foreign_key = user_id))]
#[diesel(table_name = auth)]
// #[changeset_options(treat_none_as_null = "true")]
pub struct Auths {
    pub id: i32,
    pub user_id: i32,
    pub pin: String,
    pub tries: i32,
    pub expires_at: DateTime<Utc>,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
impl Authenticable for Users {
    async fn request_auth_code(auth: AuthCodeRequest) -> Result<AuthCodeRequest, anyhow::Error> {
        // let mut conn = db::connection()?;

        // let user = Users::email(auth.email());

        let user = match Users::email(auth.email()) {
            Ok(user) => user,
            Err(err) => Users::create(User { email: auth.email().to_string(), name: auth.email().to_string() })?
        };

        // Auths::belonging_to(&user).load();

        let mut user_auth = match Auths::user(user.id) {
            Ok(uauth) => uauth,
            Err(err) => Auths::create(
                Auth {
                    user_id: user.id,
                    pin: thread_rng().gen_range(100000..999999).to_string(),
                    tries: 0,
                    expires_at: Utc::now() + Duration::minutes(1),
                    locked_until: None
                }
            )?
        };

        match user_auth.locked_until {
            None => (),
            Some(locked_until) => {
                if locked_until.gt(&Utc::now()) {
                    anyhow::bail!(format!("Account is locked until {}", locked_until.to_string()));
                }

                // Unlock account
                user_auth = Auths::unlock(&user_auth)?;
            }
        }

        // TODO: Check tries?

        // TODO: Notify user
        println!("Attempt to send mail to user [{0}]", user.email);
        Mailer::send_mail(Mailable {
            to: (user.email, user.name).into(),
            content: format!("Verification code [{}]", user_auth.pin).to_string(),
            subject: format!("Verification code [{}]", user_auth.pin)
        }).await;

        Ok(auth)
    }

    fn login(auth: AuthRequest) -> Result<AuthResponse, anyhow::Error> {
        // let mut conn = db::connection()?;

        let user = Users::email(auth.email())?;

        let mut user_auth = Auths::user(user.id)?;

        // Auths::belonging_to(&user).load();

        match user_auth.locked_until {
            None => (),
            Some(locked_until) => {
                if locked_until.gt(&Utc::now()) {
                    anyhow::bail!(format!("Account is locked until {}", locked_until.to_string()));
                }

                // Unlock account
                user_auth = Auths::unlock(&user_auth)?;
            }
        }

        if auth.pin() != user_auth.pin {
            user_auth.tries += 1;

            if user_auth.tries >= 3 {
                user_auth.locked_until = Some(Utc::now() + Duration::minutes(1));
                user_auth = Auths::update(user_auth.id, user_auth.into())?;

                anyhow::bail!(format!("Account is locked until {}", user_auth.locked_until.unwrap().to_string()));
            }

            user_auth = Auths::update(user_auth.id, user_auth.into())?;
            anyhow::bail!(format!("Invalid credentials. Tries remaining: {}", (3 - user_auth.tries).to_string()));
        }

        Ok(AuthResponse { user, token: "".to_string(), permissions: None })
    }

    fn logout() -> Result<Option<()>, anyhow::Error> {
        let mut conn = db::connection()?;

        // let user = User::from(user);
        // let user = diesel::insert_into(users::table)
        //     .values(user)
        //     .get_result(&mut conn)?;

        Ok(None)
    }

    fn profile() -> Result<Option<()>, anyhow::Error> {
        // let mut conn = db::connection()?;
        // let user = users::table.filter(users::id.eq(id)).first(&mut conn)?;
        Ok(None)
    }
}

#[async_trait]
pub trait Authenticable {
    async fn request_auth_code(auth: AuthCodeRequest) -> Result<AuthCodeRequest, anyhow::Error>;

    fn login(auth: AuthRequest) -> Result<AuthResponse, anyhow::Error>;

    fn logout() -> Result<Option<()>, anyhow::Error>;

    fn profile() -> Result<Option<()>, anyhow::Error>;
}

impl Auths {
    pub fn all() -> Result<Vec<Self>, anyhow::Error> {
        let mut conn = db::connection()?;
        let auths = auth::table.load::<Auths>(&mut conn)?;
        Ok(auths)
    }

    pub fn find(id: i32) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let auth = auth::table.filter(auth::id.eq(id)).first(&mut conn)?;
        Ok(auth)
    }

    pub fn user(user_id: i32) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let auth = auth::table.filter(auth::user_id.eq(user_id)).first(&mut conn)?;
        Ok(auth)
    }

    pub fn create(auth: Auth) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let auth = Auth::from(auth);
        let auth = diesel::insert_into(auth::table)
            .values(auth)
            .get_result(&mut conn)?;
        Ok(auth)
    }

    pub fn update(id: i32, auth: Auth) -> Result<Self, anyhow::Error> {
        let mut conn = db::connection()?;
        let user = diesel::update(auth::table)
            .filter(auth::id.eq(id))
            .set(auth)
            .get_result(&mut conn)?;
        Ok(user)
    }

    pub fn unlock(user_auth: &Auths) -> Result<Self, anyhow::Error> {
        #[derive(AsChangeset)]
        #[diesel(table_name = auth)]
        #[changeset_options(treat_none_as_null = "true")]
        struct UnlockedAuth<'a> {
            tries: &'a i32,
            locked_until: Option<&'a DateTime<Utc>>
        }
        // use self::auth::dsl::*;

        let mut conn = db::connection()?;
        let user = diesel::update(user_auth)
            .set(&UnlockedAuth{
                tries: &0,
                locked_until: None
            })
            // .set(user_auth)
            .get_result(&mut conn)?;
        Ok(user)
    }

    pub fn delete(id: i32) -> Result<usize, anyhow::Error> {
        let mut conn = db::connection()?;
        let res = diesel::delete(auth::table.filter(auth::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl Auth {
    fn from(auth: Auth) -> Auth {
        Auth {
            user_id: auth.user_id,
            pin: auth.pin,
            tries: auth.tries,
            expires_at: auth.expires_at,
            locked_until: auth.locked_until,
            // created_at: auth.created_at,
            // updated_at: auth.updated_at
        }
    }
}

impl From<Auths> for Auth {
    fn from(auth: Auths) -> Auth {
        Auth {
            user_id: auth.user_id,
            pin: auth.pin,
            tries: auth.tries,
            expires_at: auth.expires_at,
            locked_until: auth.locked_until,
            // created_at: auth.created_at,
            // updated_at: auth.updated_at
        }
    }
}
