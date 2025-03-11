use std::time::Duration;

use google_jwt::ClientAsync;
use rsweb_auth::google_client_id;
use rsweb_database::user::{UserEssentials, UserService};
use rsweb_utils::format_expiry;
use serde::Deserialize;
use warp::{Filter, reply::Reply};

use crate::filters::BadRequest;

#[derive(Debug, Deserialize)]
pub struct LoginBody {
    email: Option<String>,
    password: Option<String>,
    credential: Option<String>,
}

pub fn filter() -> impl Filter<Extract = (LoginBody,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn handle(body: LoginBody) -> Result<impl warp::Reply, warp::Rejection> {
    let essentials: UserEssentials;

    // Credential then google login
    if let Some(credential) = body.credential {
        let client = ClientAsync::new(&google_client_id().unwrap());
        let id_token = match client.verify_id_token_async(&credential).await {
            Ok(token) => token,
            Err(_) => return Err(warp::reject::custom(BadRequest)),
        };

        let details = match UserService::get_google_user_details(&id_token.claims.subject).await {
            Ok(d) => d,
            Err(_) => return Err(warp::reject::custom(BadRequest)),
        };

        if details.banned {
            return Err(warp::reject::custom(BadRequest));
        }

        essentials = UserEssentials {
            id: details.id,
            email: details.email,
            role: details.role,
            handle: details.handle,
        }
    } else {
        let email = match body.email {
            Some(email) => email,
            None => return Err(warp::reject::custom(BadRequest)),
        };

        let password = match body.password {
            Some(password) => password,
            None => return Err(warp::reject::custom(BadRequest)),
        };

        if !email.contains('@') || password.len() < 6 && password.len() > 64 {
            return Err(warp::reject::custom(BadRequest));
        }

        let details = match UserService::get_user_details(&email).await {
            Ok(d) => d,
            Err(_) => return Err(warp::reject::custom(BadRequest)),
        };

        if details.banned {
            return Err(warp::reject::custom(BadRequest));
        }

        let stored_pwd = match details.password {
            Some(pwd) => pwd,
            None => return Err(warp::reject::custom(BadRequest)),
        };

        let stored_salt = match details.password_salt {
            Some(salt) => salt,
            None => return Err(warp::reject::custom(BadRequest)),
        };

        match rsweb_crypto::hash::cmp_password_hash(&password, &stored_pwd, &stored_salt) {
            Ok(b) => {
                if !b {
                    return Err(warp::reject::custom(BadRequest));
                }
            }
            Err(_) => return Err(warp::reject::custom(BadRequest)),
        }

        essentials = UserEssentials {
            id: details.id,
            email: details.email,
            role: details.role,
            handle: details.handle,
        }
    }

    let at = rsweb_auth::claims::Claims::from_user_essentials(&essentials)
        .await
        .create_token()
        .await;
    let rt = match rsweb_auth::claims::refresh_tokens::create(essentials.id).await {
        Ok(rt) => rt,
        Err(_) => return Err(warp::reject::custom(BadRequest)),
    };

    let res = warp::reply();

    // Set auth token expiration (30 minutes)
    let auth_expires = format_expiry(Duration::from_secs(30 * 60));
    // Set refresh token expiration (2 weeks)
    let refresh_expires = format_expiry(Duration::from_secs(14 * 24 * 60 * 60));

    let mut cookies = warp::http::header::HeaderMap::new();
    cookies.append(
        "Set-Cookie",
        format!(
            "auth_token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Expires={}; Max-Age={}",
            at,
            auth_expires,
            30 * 60
        )
        .parse()
        .unwrap(),
    );
    cookies.append(
        "Set-Cookie",
        format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Path=/; Expires={}; Max-Age={}",
            rt,
            refresh_expires,
            14 * 24 * 60 * 60
        )
        .parse()
        .unwrap(),
    );

    let mut response = res.into_response();
    let headers = response.headers_mut();
    headers.extend(cookies);

    Ok(response)
}
