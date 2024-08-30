mod chat;
mod chat_participant;
mod user;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Database(PgPool);

impl Database {
    pub const fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}
