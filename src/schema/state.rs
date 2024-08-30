mod builder;

use std::sync::Arc;

use builder::AppStateBuilder;

use super::Database;

#[derive(Debug, Clone)]
pub struct AppState(Arc<AppStateInner>);

#[derive(Debug)]
struct AppStateInner {
    database: Database,
}

impl AppState {
    pub const fn builder() -> AppStateBuilder {
        AppStateBuilder::new()
    }
}

impl AppState {
    pub fn database(&self) -> &Database {
        &self.0.database
    }
}
