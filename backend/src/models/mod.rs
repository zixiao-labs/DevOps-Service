use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: Uuid,
    pub owner: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub language_color: String,
    pub stars: u32,
    pub forks: u32,
    pub is_private: bool,
    pub topics: Vec<String>,
    pub default_branch: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub repo: String,
    pub title: String,
    pub labels: Vec<Label>,
    pub author: String,
    pub assignee: Option<String>,
    pub comments: u32,
    pub open: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    pub id: u64,
    pub repo: String,
    pub title: String,
    pub source_branch: String,
    pub target_branch: String,
    pub author: String,
    pub reviewers: Vec<String>,
    pub approvals: u32,
    pub comments: u32,
    pub status: MergeRequestStatus,
    pub ci_status: CIStatus,
    pub labels: Vec<Label>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MergeRequestStatus {
    Open,
    Merged,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CIStatus {
    Success,
    Running,
    Failed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: u64,
    pub repo: String,
    pub branch: String,
    pub commit: String,
    pub commit_message: String,
    pub author: String,
    pub status: CIStatus,
    pub stages: Vec<PipelineStage>,
    pub duration: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub name: String,
    pub status: CIStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub name: String,
    pub role: String,
    pub email: String,
    pub avatar: String,
    pub joined_at: String,
    pub repo_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub repo_count: u32,
    pub open_issues: u32,
    pub open_merge_requests: u32,
    pub pipeline_pass_rate: f32,
}
