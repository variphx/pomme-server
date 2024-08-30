use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};

use crate::schema::{AppState, ChatCreatePayload, ChatQueryBySearchParams, ChatQueryParams};

pub async fn create_chat(
    State(state): State<AppState>,
    Json(payload): Json<ChatCreatePayload>,
) -> impl IntoResponse {
    state.database().insert_chat(payload.into()).await
}

pub async fn query_chat_by_id(
    State(state): State<AppState>,
    Path(params): Path<i64>,
) -> impl IntoResponse {
    query_chat(state, params).await
}

pub async fn query_chat_by_search(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    Query(search_params): Query<ChatQueryBySearchParams>,
) -> impl IntoResponse {
    query_chat(state, (user_id, search_params)).await
}

async fn query_chat(state: AppState, params: impl Into<ChatQueryParams>) -> impl IntoResponse {
    state.database().query_chat(params.into()).await
}
