use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewUser {
    username: String,
    email: String,
    password_hash: String,
}

impl NewUser {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct UserCreatePayload {
    username: String,
    email: String,
    password: String,
}

impl TryFrom<UserCreatePayload> for NewUser {
    type Error = Error;
    fn try_from(value: UserCreatePayload) -> Result<Self, Self::Error> {
        use argon2::PasswordHasher;

        let saltstring = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(value.password.as_bytes(), &saltstring)
            .map_err(|e| Self::Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))?
            .to_string();

        Ok(Self {
            username: value.username,
            email: value.email,
            password_hash,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserQueryResult {
    SingleUser(User),
    MultipleUsers(Vec<User>),
}

impl IntoResponse for UserQueryResult {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::SingleUser(value) => Json(value).into_response(),
            Self::MultipleUsers(value) => Json(value).into_response(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserQueryParams {
    ById(i64),
    BySearching(UserQueryBySearchingParams),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct UserQueryBySearchingParams {
    pub search: String,
    pub limit: usize,
    pub offset: usize,
}

impl From<i64> for UserQueryParams {
    fn from(value: i64) -> Self {
        Self::ById(value)
    }
}

impl From<UserQueryBySearchingParams> for UserQueryParams {
    fn from(value: UserQueryBySearchingParams) -> Self {
        Self::BySearching(value)
    }
}
