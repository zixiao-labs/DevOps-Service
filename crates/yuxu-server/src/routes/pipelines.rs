use crate::error::AppError;
pub async fn list(Path(_full_name): Path<String>) -> Result<Json<ListPipelinesResponse>, AppError> {
    Err(AppError::BadRequest("not implemented".into()))
}
pub async fn trigger(Path(_full_name): Path<String>) -> Result<Json<Pipeline>, AppError> {
    Err(AppError::BadRequest("not implemented".into()))
}
