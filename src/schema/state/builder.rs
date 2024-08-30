use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;

use crate::schema::Database;

use super::{AppState, AppStateInner};

#[derive(Debug, Clone)]
pub struct AppStateBuilder {
    database_url: Option<String>,
    database_pool_options: Option<PgPoolOptions>,
}

impl AppStateBuilder {
    pub const fn new() -> Self {
        Self {
            database_url: None,
            database_pool_options: None,
        }
    }

    pub fn with_database_url(mut self, database_url: String) -> Self {
        self.database_url = Some(database_url);
        self
    }

    pub fn with_database_pool_options(mut self, database_pool_options: PgPoolOptions) -> Self {
        self.database_pool_options = Some(database_pool_options);
        self
    }

    pub async fn build(self) -> AppState {
        let database = {
            let pool = {
                let url = match self.database_url {
                    Some(url) => url,
                    None => {
                        dotenvy::dotenv().ok();
                        std::env::var("DATABASE_URL")
                            .expect("`DATABASE_URL environment variable has not been set`")
                    }
                };

                let pool_options = match self.database_pool_options {
                    Some(options) => options,
                    None => PgPoolOptions::new(),
                };

                pool_options.connect(&url).await
            }
            .expect("Database connections pool has failed to established");

            Database::new(pool)
        };

        AppState(Arc::new(AppStateInner { database }))
    }
}
