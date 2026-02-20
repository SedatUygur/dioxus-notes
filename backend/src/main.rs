use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use core::{Memo};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde_json::json;
use tracing_subscriber::{fmt, EnvFilter};
/*
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use tower_http::cors::{Any, CorsLayer};
*/

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

impl From<DbMemo> for Memo {
    fn from(d: DbMemo) -> Self {
        Memo {
            id: d.id,
            title: d.title,
            content: d.content,
            public: d.public,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }
    }
}

async fn main() {
    println!("Hello, world!");
}

async fn list_memos(State(state): State<AppState>) -> impl IntoResponse {
    let rows = sqlx::query_as::<_, DbMemo>("SELECT * FROM memos ORDER BY created_at DESC LIMIT 100")
        .fetch_all(&state.db)
        .await;

    match rows {
        Ok(items) => (StatusCode::OK, Json(items.into_iter().map(|r| Memo::from(r)).collect::<Vec<_>>())),
        Err(e) => {
            tracing::error!("db error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"db error"})))
        }
    }
}

async fn get_memo(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let rec = sqlx::query_as::<_, DbMemo>("SELECT * FROM memos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await;

    match rec {
        Ok(row) => (StatusCode::OK, Json(Memo::from(row))),
        Err(sqlx::Error::RowNotFound) => (StatusCode::NOT_FOUND, Json(json!({"error":"not found"}))),
        Err(e) => {
            tracing::error!("db error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"db error"})))
        }
    }
}

async fn search_memos(State(state): State<AppState>, axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>) -> impl IntoResponse {
    let q = params.get("q").map(|s| s.as_str()).unwrap_or("");
    // simple ILIKE search on title and content
    let pattern = format!("%{}%", q);
    let rows = sqlx::query_as::<_, DbMemo>("SELECT * FROM memos WHERE title ILIKE $1 OR content ILIKE $1 ORDER BY created_at DESC LIMIT 100")
        .bind(pattern)
        .fetch_all(&state.db)
        .await;

    match rows {
        Ok(items) => (StatusCode::OK, Json(items.into_iter().map(|r| Memo::from(r)).collect::<Vec<_>>())),
        Err(e) => {
            tracing::error!("search error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"search failed"})))
        }
    }
}