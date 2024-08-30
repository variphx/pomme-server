use axum::http::StatusCode;

use crate::{
    schema::{NewUser, User, UserQueryBySearchingParams, UserQueryParams, UserQueryResult},
    Error,
};

use super::Database;

impl Database {
    pub async fn insert_user(&self, user: NewUser) -> Result<StatusCode, Error> {
        sqlx::query_file!(
            "queries/insert_user.sql",
            user.username(),
            user.email(),
            user.password_hash()
        )
        .execute(&self.0)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))
    }

    pub async fn query_user(&self, query: UserQueryParams) -> Result<UserQueryResult, Error> {
        match query {
            UserQueryParams::ById(user_id) => self.query_user_by_id(user_id).await,
            UserQueryParams::BySearching(params) => self.query_user_by_search(params).await,
        }
    }

    async fn query_user_by_id(&self, user_id: i64) -> Result<UserQueryResult, Error> {
        sqlx::query_file_as!(User, "queries/query_user_by_id.sql", user_id)
            .fetch_optional(&self.0)
            .await
            .map(|user| {
                user.map(UserQueryResult::SingleUser).ok_or_else(|| {
                    Error::new(
                        StatusCode::NOT_FOUND,
                        format!("Could not find user with `user_id`: {}", user_id),
                    )
                })
            })
            .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))?
    }

    async fn query_user_by_search(
        &self,
        UserQueryBySearchingParams {
            search,
            offset,
            limit,
        }: UserQueryBySearchingParams,
    ) -> Result<UserQueryResult, Error> {
        sqlx::query_file_as!(
            User,
            "queries/query_user_by_search.sql",
            format!("%{search}%"),
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.0)
        .await
        .map(UserQueryResult::MultipleUsers)
        .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}
