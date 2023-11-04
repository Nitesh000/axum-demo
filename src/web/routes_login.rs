
use axum::{Json, Router, routing::post};
use jsonwebtoken::{Header, encode, EncodingKey};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie, cookie::{Expiration, time::{OffsetDateTime, Duration}}};

use crate::{Error, Result, web::AUTH_TOKEN};

use super::JWT_SECRET;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("-->> {:<12} - api_login", "HANLDER");

    // TODO: implement real db/auth logic.
    if payload.username != "nitesh000" || payload.pwd != "nkt" {
        return Err(Error::LoginFail)
    }

    // FIXME: Implement real auth-token gneerating the signature.
    // NOTE: done; 
    let token = create_jwt(payload)?;
    let exp = Expiration::DateTime(OffsetDateTime::now_utc() + Duration::weeks(1));
    let c = Cookie::build(AUTH_TOKEN, token).expires(exp).http_only(true).secure(true).finish();
    cookies.add(c);
   
    // create the success body
    let body = Json(json!({
    "result": {
    "success": true
    }
    }));

    Ok(body)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginPayload {
    username: String,
    pwd: String,
}

fn create_jwt(payload: Json<LoginPayload>) -> Result<String> {
    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let claims = LoginPayload {
        username: payload.username.clone(),
        pwd: payload.pwd.clone(),
    };
    encode(&header, &claims,&EncodingKey::from_secret(JWT_SECRET.as_ref())).map_err(|_| Error::JWTTokenCreationError)
} 
