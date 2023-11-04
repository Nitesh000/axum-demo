use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    LoginFail,
    JWTTokenCreationError,                  // token creation error
    AuthFailNoAuthTokenCookie,              // auth error
    TicketDeleteFailIdNotFound { id: u64 }, // modal errors
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("-->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
