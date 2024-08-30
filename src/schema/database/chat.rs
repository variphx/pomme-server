use axum::http::StatusCode;

use crate::{
    schema::{Chat, ChatQueryBySearchParams, ChatQueryParams, ChatQueryResult, NewChat},
    Error,
};

use super::Database;

impl Database {
    pub async fn insert_chat(&self, chat: NewChat) -> Result<StatusCode, Error> {
        let chat_id = sqlx::query_file_scalar!("queries/insert_chat.sql", chat.chat_name())
            .fetch_one(&self.0)
            .await
            .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))?;

        for new_chat_participant in chat.new_chat_participants().iter().cloned() {
            let database = self.clone();
            tokio::spawn(async move {
                database
                    .insert_chat_participant(chat_id, new_chat_participant)
                    .await
                    .expect("Insertion of `chat_participant` has failed")
            });
        }

        Ok(StatusCode::CREATED)
    }

    pub async fn query_chat(&self, params: ChatQueryParams) -> Result<ChatQueryResult, Error> {
        match params {
            ChatQueryParams::ById(chat_id) => self.query_chat_by_id(chat_id).await,
            ChatQueryParams::BySearching((user_id, search_params)) => {
                self.query_chat_by_search(user_id, search_params).await
            }
        }
    }

    async fn query_chat_by_id(&self, chat_id: i64) -> Result<ChatQueryResult, Error> {
        sqlx::query_file_as!(Chat, "queries/query_chat_by_id.sql", chat_id)
            .fetch_one(&self.0)
            .await
            .map(ChatQueryResult::Single)
            .map_err(|e| Error::new(StatusCode::NOT_FOUND, e))
    }

    async fn query_chat_by_search(
        &self,
        user_id: i64,
        ChatQueryBySearchParams {
            search,
            limit,
            offset,
        }: ChatQueryBySearchParams,
    ) -> Result<ChatQueryResult, Error> {
        sqlx::query_file_as!(
            Chat,
            "queries/query_chat_by_search.sql",
            user_id,
            search,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.0)
        .await
        .map(ChatQueryResult::Multiple)
        .map_err(|e| Error::new(StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}
