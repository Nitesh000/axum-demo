use crate::{Error, Result, web::AUTH_TOKEN};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};

pub fn routes() -> Router {
    Router::new().route("/api/login", post( api_login ))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: Implement real db/auth logic.
    if payload.usernmae != "nitesh" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    //FIXME: This is a dummy token. Implement real auth-token genneration/signature.
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // NOTE: Create the success body.
    let body = Json(json!({
        "result": {
        "success": true
    }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayLoad {
    usernmae: String,
    pwd: String,
}
