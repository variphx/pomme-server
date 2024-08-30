#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ChatParticipant {
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewChatParticipant {
    user_id: i64,
}

impl NewChatParticipant {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct ChatParticipantCreatePayload {
    user_id: i64,
}

impl From<ChatParticipantCreatePayload> for NewChatParticipant {
    fn from(value: ChatParticipantCreatePayload) -> Self {
        Self {
            user_id: value.user_id,
        }
    }
}
