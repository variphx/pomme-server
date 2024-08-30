use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    status_code: StatusCode,
    info: String,
}

impl Error {
    pub fn new(status_code: StatusCode, info: impl ToString) -> Self {
        Self {
            status_code,
            info: info.to_string(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.info).into_response()
    }
}
