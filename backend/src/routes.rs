use axum::{Json, extract::Path};
use core::Memo;

async fn list_memos() -> Json<Vec<Memo>> {
  // query sqlite via sqlx (omitted)
  Json(vec![])
}