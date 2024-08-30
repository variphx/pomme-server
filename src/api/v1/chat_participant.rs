use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::schema::{AppState, ChatParticipantCreatePayload};

pub async fn create_chat_participant(
    State(state): State<AppState>,
    Path(chat_id): Path<i64>,
    Json(payload): Json<ChatParticipantCreatePayload>,
) -> impl IntoResponse {
    state
        .database()
        .insert_chat_participant(chat_id, payload.into())
        .await
}

pub async fn query_chat_participants_of_chat(
    State(state): State<AppState>,
    Path(chat_id): Path<i64>,
) -> impl IntoResponse {
    state
        .database()
        .query_chat_participants_of_chat(chat_id)
        .await
}
