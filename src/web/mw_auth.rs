use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, Validation, EncodingKey, DecodingKey};
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

use super::JWT_SECRET;
use super::routes_login::LoginPayload;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("-->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: Real auth-token parsing
    // done
    let result = parse_token(auth_token);
    if !result {
        return Err(Error::AuthFailNoAuthTokenCookie);
    }
    Ok(next.run(req).await)
}

// Parse a token of format `user-[user-id].[expiration].[signature]`
// returns (user_id, expiration, signature)

fn parse_token(token: Option<String>) -> bool {
    match token {
        Some(val) => {
            decode::<LoginPayload>(&val, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &Validation::new(jsonwebtoken::Algorithm::HS256)).map_err(|_| Error::AuthFailNoAuthTokenCookie);
            return true;
        }
        None => {
            return false;
        }
    }
}
