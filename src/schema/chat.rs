use axum::{response::IntoResponse, Json};
use time::OffsetDateTime;

use super::{ChatParticipantCreatePayload, NewChatParticipant};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, sqlx::FromRow)]
pub struct Chat {
    pub chat_id: i64,
    pub chat_name: String,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewChat {
    chat_name: String,
    new_chat_participants: Vec<NewChatParticipant>,
}

impl NewChat {
    pub fn chat_name(&self) -> &str {
        &self.chat_name
    }

    pub fn new_chat_participants(&self) -> &[NewChatParticipant] {
        &self.new_chat_participants
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct ChatCreatePayload {
    chat_name: String,
    chat_participant_create_payloads: Vec<ChatParticipantCreatePayload>,
}

impl From<ChatCreatePayload> for NewChat {
    fn from(value: ChatCreatePayload) -> Self {
        Self {
            chat_name: value.chat_name,
            new_chat_participants: value
                .chat_participant_create_payloads
                .into_iter()
                .map(NewChatParticipant::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatQueryParams {
    ById(i64),
    BySearching((i64, ChatQueryBySearchParams)),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct ChatQueryBySearchParams {
    pub search: String,
    pub limit: usize,
    pub offset: usize,
}

impl From<i64> for ChatQueryParams {
    fn from(value: i64) -> Self {
        Self::ById(value)
    }
}

impl From<(i64, ChatQueryBySearchParams)> for ChatQueryParams {
    fn from(value: (i64, ChatQueryBySearchParams)) -> Self {
        Self::BySearching(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatQueryResult {
    Single(Chat),
    Multiple(Vec<Chat>),
}

impl IntoResponse for ChatQueryResult {
    fn into_response(self) -> axum::response::Response {
        match self {
            ChatQueryResult::Single(chat) => Json(chat).into_response(),
            ChatQueryResult::Multiple(chats) => Json(chats).into_response(),
        }
    }
}
