mod api;
mod error;
mod schema;

use api::v1;
use axum::{routing, Router};
use error::*;
use schema::AppState;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = {
        let database_url = {
            dotenvy::dotenv().ok();
            std::env::var("DATABASE_URL")
                .expect("`DATABASE_URL` environment variable should have been set")
        };

        AppState::builder()
            .with_database_url(database_url)
            .with_database_pool_options(PgPoolOptions::new())
            .build()
            .await
    };

    let app = Router::new()
        .route(
            "/api/v1/users",
            routing::get(v1::query_user_by_search).post(v1::create_user),
        )
        .route("/api/v1/users/:user_id", routing::get(v1::query_user_by_id))
        .route("/api/v1/users/chats", routing::post(v1::create_chat))
        .route(
            "/api/v1/users/:user_id/chats",
            routing::get(v1::query_chat_by_search),
        )
        .route(
            "/api/v1/users/chats/:chat_id",
            routing::get(v1::query_chat_by_id),
        )
        .route(
            "/api/v1/users/chats/:chat_id/participants",
            routing::get(v1::query_chat_participants_of_chat).post(v1::create_chat_participant),
        )
        .with_state(state);

    let addr = "[::1]:8080";
    let tcp_listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(tcp_listener, app).await.unwrap();
}
