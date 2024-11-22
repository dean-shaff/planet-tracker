use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::astronomy::Planet;

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Deserialize)]
pub struct GetAstronObjectParams {
    pub name: Planet,
    pub lon: f64,
    pub lat: f64,
    pub elevation: f64,
    pub when: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct GetAstronObjectResponse {
    pub name: Planet,
    pub magnitude: f64,
    pub size: f64,
    pub az: f64,
    pub el: f64,
    pub ra: f64,
    pub dec: f64,
    pub setting_time: DateTime<Utc>,
    pub rising_time: DateTime<Utc>,
    pub when: DateTime<Utc>,
}
