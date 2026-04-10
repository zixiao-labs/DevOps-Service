mod repos;
mod issues;
mod merge_requests;
mod pipelines;
mod members;
mod dashboard;

pub use repos::repos_router;
pub use issues::issues_router;
pub use merge_requests::merge_requests_router;
pub use pipelines::pipelines_router;
pub use members::members_router;
pub use dashboard::dashboard_router;
