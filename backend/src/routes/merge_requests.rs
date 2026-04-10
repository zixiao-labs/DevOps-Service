use axum::{routing::get, Json, Router};
use chrono::Utc;

use crate::models::{CIStatus, Label, MergeRequest, MergeRequestStatus};

fn mock_merge_requests() -> Vec<MergeRequest> {
    let now = Utc::now();
    vec![
        MergeRequest {
            id: 142,
            repo: "logos".into(),
            title: "修复 WebSocket 重连逻辑".into(),
            source_branch: "fix/ws-reconnect".into(),
            target_branch: "main".into(),
            author: "A".into(),
            reviewers: vec!["C".into(), "D".into()],
            approvals: 2,
            comments: 7,
            status: MergeRequestStatus::Merged,
            ci_status: CIStatus::Success,
            labels: vec![Label { name: "bug".into(), color: "#f85149".into() }],
            created_at: now,
        },
        MergeRequest {
            id: 67,
            repo: "aefanyl".into(),
            title: "添加 OIDC 认证支持".into(),
            source_branch: "feat/oidc".into(),
            target_branch: "main".into(),
            author: "D".into(),
            reviewers: vec!["A".into()],
            approvals: 1,
            comments: 14,
            status: MergeRequestStatus::Open,
            ci_status: CIStatus::Success,
            labels: vec![
                Label { name: "功能".into(), color: "#3fb950".into() },
                Label { name: "协作".into(), color: "#7c4dff".into() },
            ],
            created_at: now,
        },
        MergeRequest {
            id: 46,
            repo: "chen-the-dawnstreak".into(),
            title: "SSR 流式渲染支持".into(),
            source_branch: "feat/streaming-ssr".into(),
            target_branch: "main".into(),
            author: "A".into(),
            reviewers: vec!["C".into()],
            approvals: 0,
            comments: 3,
            status: MergeRequestStatus::Open,
            ci_status: CIStatus::Running,
            labels: vec![Label { name: "功能".into(), color: "#3fb950".into() }],
            created_at: now,
        },
        MergeRequest {
            id: 66,
            repo: "aefanyl".into(),
            title: "优化 Protobuf 消息序列化性能".into(),
            source_branch: "perf/protobuf".into(),
            target_branch: "main".into(),
            author: "C".into(),
            reviewers: vec!["A".into(), "D".into()],
            approvals: 1,
            comments: 5,
            status: MergeRequestStatus::Open,
            ci_status: CIStatus::Failed,
            labels: vec![Label { name: "性能".into(), color: "#d29922".into() }],
            created_at: now,
        },
    ]
}

async fn list_merge_requests() -> Json<Vec<MergeRequest>> {
    Json(mock_merge_requests())
}

pub fn merge_requests_router() -> Router {
    Router::new().route("/", get(list_merge_requests))
}
