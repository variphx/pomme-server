use axum::{http::StatusCode, Json};

use crate::{
    schema::{ChatParticipant, NewChatParticipant},
    Error,
};

use super::Database;

impl Database {
    pub async fn insert_chat_participant(
        &self,
        chat_id: i64,
        chat_participant: NewChatParticipant,
    ) -> Result<StatusCode, Error> {
        sqlx::query_file!(
            "queries/insert_chat_participant.sql",
            chat_participant.user_id(),
            chat_id
        )
        .execute(&self.0)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))
    }

    pub async fn query_chat_participants_of_chat(
        &self,
        chat_id: i64,
    ) -> Result<Json<Vec<ChatParticipant>>, Error> {
        sqlx::query_file_as!(
            ChatParticipant,
            "queries/query_chat_participants_of_chat.sql",
            chat_id
        )
        .fetch_all(&self.0)
        .await
        .map(Json)
        .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}
