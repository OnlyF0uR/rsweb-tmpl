use google_jwt::ClientAsync;
use rsweb_auth::{self, google_client_id};
use rsweb_database::user::UserEssentials;
use serde::Deserialize;
use warp::{Filter, reply::Reply};

use crate::filters::BadRequest;

#[derive(Debug, Deserialize)]
pub struct SignupBody {
    username: String,
    email: Option<String>,
    password: Option<String>,
    credential: Option<String>,
}

pub fn filter() -> impl Filter<Extract = (SignupBody,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn handle(body: SignupBody) -> Result<impl warp::Reply, warp::Rejection> {
    let essentials: UserEssentials;

    // Credential then google login
    if let Some(credential) = body.credential {
        let client = ClientAsync::new(&google_client_id().unwrap());
        let id_token = match client.verify_id_token_async(&credential).await {
            Ok(token) => token,
            Err(_) => return Err(warp::reject::custom(BadRequest)),
        };

        // TODO: Here we need to register the (google) user
        todo!()
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

        // TODO: Here we need to register the (email & pwd) user
        todo!()
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

    let mut cookies = warp::http::header::HeaderMap::new();
    cookies.append(
        "Set-Cookie",
        format!(
            "auth_token={}; HttpOnly; Secure; SameSite=Strict; Path=/",
            at
        )
        .parse()
        .unwrap(),
    );
    cookies.append(
        "Set-Cookie",
        format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Path=/",
            rt
        )
        .parse()
        .unwrap(),
    );

    let mut response = res.into_response();
    let headers = response.headers_mut();
    headers.extend(cookies);

    Ok(response)
}
