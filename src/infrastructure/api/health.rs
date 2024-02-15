use crate::domain::model::health::HealthStatus;

use actix_web::{
    get, 
    HttpResponse,
    web::Json,
    http::{header::ContentType, StatusCode},
    error::ResponseError,
};

use derive_more::{Display};

#[derive(Debug, Display)]
pub enum HealthStatusError {
    InternalFailure,
    BadRequest,
}

impl ResponseError for HealthStatusError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            HealthStatusError::InternalFailure => StatusCode::FAILED_DEPENDENCY,
            HealthStatusError::BadRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/health")]
pub async fn get_health() -> Result<Json<HealthStatus>, HealthStatusError> {
    Ok(Json(HealthStatus::new(String::from("HEALTHY"))))
}
