use axum::response::IntoResponse;

use crate::response::resp::ApiResponse;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Method Not Allowed")]
    MethodNotAllowed,
    #[error("{0}")]
    Biz(String),
    #[error("尚未授权：{0}")]
    Unauthenticated(String),
}

impl ApiError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Biz(_) => axum::http::StatusCode::OK,
            ApiError::Unauthenticated(_) => axum::http::StatusCode::UNAUTHORIZED,
        }
    }
}

/// 将错误转换为响应
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code(),
            axum::Json(ApiResponse::<()>::err(self.to_string())),
        )
            .into_response()
    }
}

/// 把错误转换成返回值
impl From<ApiError> for axum::http::Response<axum::body::Body> {
    fn from(error: ApiError) -> Self {
        error.into_response()
    }
}
