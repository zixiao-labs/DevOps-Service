use axum::{extract::Path, routing::get, Json, Router};
use chrono::Utc;
use uuid::Uuid;

use crate::models::Repository;

fn mock_repos() -> Vec<Repository> {
    let now = Utc::now();
    vec![
        Repository {
            id: Uuid::new_v4(),
            owner: "zixiao-labs".into(),
            name: "logos".into(),
            description: "桌面代码编辑器，基于 Electron + Vue 3 + Monaco Editor".into(),
            language: "TypeScript".into(),
            language_color: "#3178c6".into(),
            stars: 342,
            forks: 28,
            is_private: false,
            topics: vec!["editor".into(), "electron".into(), "vue3".into()],
            default_branch: "main".into(),
            created_at: now,
            updated_at: now,
        },
        Repository {
            id: Uuid::new_v4(),
            owner: "zixiao-labs".into(),
            name: "chen-the-dawnstreak".into(),
            description: "轻量级 React 元框架".into(),
            language: "TypeScript".into(),
            language_color: "#3178c6".into(),
            stars: 156,
            forks: 12,
            is_private: false,
            topics: vec!["react".into(), "framework".into(), "ssr".into()],
            default_branch: "main".into(),
            created_at: now,
            updated_at: now,
        },
        Repository {
            id: Uuid::new_v4(),
            owner: "zixiao-labs".into(),
            name: "aefanyl".into(),
            description: "跨编辑器协作协议桥接".into(),
            language: "Rust".into(),
            language_color: "#dea584".into(),
            stars: 89,
            forks: 5,
            is_private: false,
            topics: vec!["collaboration".into(), "protocol".into()],
            default_branch: "main".into(),
            created_at: now,
            updated_at: now,
        },
        Repository {
            id: Uuid::new_v4(),
            owner: "zixiao-labs".into(),
            name: "nasti".into(),
            description: "基于 Rust 的高性能前端打包工具".into(),
            language: "Rust".into(),
            language_color: "#dea584".into(),
            stars: 67,
            forks: 3,
            is_private: false,
            topics: vec!["bundler".into(), "rust".into()],
            default_branch: "main".into(),
            created_at: now,
            updated_at: now,
        },
        Repository {
            id: Uuid::new_v4(),
            owner: "zixiao-labs".into(),
            name: "yuxu".into(),
            description: "DevOps Service 平台（玉虚宫）".into(),
            language: "TypeScript".into(),
            language_color: "#3178c6".into(),
            stars: 0,
            forks: 0,
            is_private: true,
            topics: vec!["devops".into(), "platform".into(), "crdt".into()],
            default_branch: "main".into(),
            created_at: now,
            updated_at: now,
        },
    ]
}

async fn list_repos() -> Json<Vec<Repository>> {
    Json(mock_repos())
}

async fn get_repo(Path((owner, name)): Path<(String, String)>) -> Json<Option<Repository>> {
    let repos = mock_repos();
    let repo = repos.into_iter().find(|r| r.owner == owner && r.name == name);
    Json(repo)
}

pub fn repos_router() -> Router {
    Router::new()
        .route("/", get(list_repos))
        .route("/{owner}/{name}", get(get_repo))
}
