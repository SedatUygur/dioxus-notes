use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{Utc, DateTime};
/* use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde_json::json;
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{fmt, EnvFilter};
use core::{Memo, NewMemo}; */

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, FromRow)]
struct DbMemo {
    id: Uuid,
    title: Option<String>,
    content: String,
    public: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
