use axum::{routing::get, Json, Router};
use crate::models::DashboardStats;

async fn get_stats() -> Json<DashboardStats> {
    Json(DashboardStats {
        repo_count: 12,
        open_issues: 34,
        open_merge_requests: 7,
        pipeline_pass_rate: 94.0,
    })
}

pub fn dashboard_router() -> Router {
    Router::new().route("/stats", get(get_stats))
}
