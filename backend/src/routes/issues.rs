use axum::{routing::get, Json, Router};
use chrono::Utc;

use crate::models::{Issue, Label};

fn mock_issues() -> Vec<Issue> {
    let now = Utc::now();
    vec![
        Issue {
            id: 89,
            repo: "aefanyl".into(),
            title: "CRDT 同步延迟优化".into(),
            labels: vec![
                Label { name: "性能".into(), color: "#d29922".into() },
                Label { name: "优先".into(), color: "#f85149".into() },
            ],
            author: "C".into(),
            assignee: Some("A".into()),
            comments: 5,
            open: true,
            created_at: now,
        },
        Issue {
            id: 234,
            repo: "logos".into(),
            title: "Monaco Editor 中文输入法候选词位置偏移".into(),
            labels: vec![Label { name: "bug".into(), color: "#f85149".into() }],
            author: "D".into(),
            assignee: Some("C".into()),
            comments: 3,
            open: true,
            created_at: now,
        },
        Issue {
            id: 88,
            repo: "aefanyl".into(),
            title: "WebSocket 断线重连后状态丢失".into(),
            labels: vec![
                Label { name: "bug".into(), color: "#f85149".into() },
                Label { name: "协作".into(), color: "#7c4dff".into() },
            ],
            author: "A".into(),
            assignee: Some("A".into()),
            comments: 8,
            open: true,
            created_at: now,
        },
        Issue {
            id: 45,
            repo: "chen-the-dawnstreak".into(),
            title: "支持文件路由的 catch-all 模式".into(),
            labels: vec![Label { name: "功能".into(), color: "#3fb950".into() }],
            author: "A".into(),
            assignee: None,
            comments: 2,
            open: true,
            created_at: now,
        },
        Issue {
            id: 87,
            repo: "aefanyl".into(),
            title: "添加 OIDC 认证支持".into(),
            labels: vec![
                Label { name: "功能".into(), color: "#3fb950".into() },
                Label { name: "协作".into(), color: "#7c4dff".into() },
            ],
            author: "D".into(),
            assignee: Some("D".into()),
            comments: 12,
            open: false,
            created_at: now,
        },
    ]
}

async fn list_issues() -> Json<Vec<Issue>> {
    Json(mock_issues())
}

pub fn issues_router() -> Router {
    Router::new().route("/", get(list_issues))
}
