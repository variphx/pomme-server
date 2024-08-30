use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    schema::{
        AppState, UserCreatePayload, UserQueryBySearchingParams, UserQueryParams, UserQueryResult,
    },
    Error,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<UserCreatePayload>,
) -> impl IntoResponse {
    state
        .database()
        .insert_user(payload.try_into()?)
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn query_user_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> impl IntoResponse {
    query_user(state, user_id).await
}

pub async fn query_user_by_search(
    State(state): State<AppState>,
    Query(search): Query<UserQueryBySearchingParams>,
) -> Result<UserQueryResult, Error> {
    query_user(state, search).await
}

async fn query_user(
    state: AppState,
    query_params: impl Into<UserQueryParams>,
) -> Result<UserQueryResult, Error> {
    state.database().query_user(query_params.into()).await
}
