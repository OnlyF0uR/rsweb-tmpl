use warp::{Filter, reject::Rejection, reply::Reply};

use crate::{
    filters::{self, blog::ensure_blog},
    pages,
};

// Example of an authenticated route
pub fn authenticated()
-> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("authenticated")
        .and(warp::get())
        .and(filters::cookies::with_auth().and_then(
            |auth_session: rsweb_auth::claims::AuthSession| async move {
                let reply = warp::reply::html(
                    "You are authenticated! This is a protected route.".to_string(),
                );

                if let Some(cookies) = cookie_map(auth_session.updated_tokens) {
                    let mut response = reply.into_response();
                    let headers = response.headers_mut();
                    headers.extend(cookies);

                    return Ok::<_, Rejection>(response);
                }

                Ok::<_, Rejection>(reply.into_response())
            },
        ))
}

pub fn explore() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("about")
        .and(warp::get())
        .map(|| warp::reply::html(pages::about::render().into_string()))
}

pub fn blog() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("blog")
        .and(warp::get())
        .and(ensure_blog().and_then(|blog| async move {
            Ok::<_, Rejection>(warp::reply::html(
                pages::blog::render(&blog).await.into_string(),
            ))
        }))
}

pub fn login() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::get())
        .and(filters::cookies::without_auth())
        .map(|| warp::reply::html(pages::portal::render().into_string()))
}

// The root route
pub fn root() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and(filters::cookies::with_opt_auth().and_then(
            |auth_session: Option<rsweb_auth::claims::AuthSession>| async move {
                let claims = auth_session.as_ref().map(|session| &session.claims);
                let reply = warp::reply::html(pages::root::home(claims).await.into_string());

                if let Some(auth_session) = auth_session {
                    if let Some(cookies) = cookie_map(auth_session.updated_tokens) {
                        let mut response = reply.into_response();
                        let headers = response.headers_mut();
                        headers.extend(cookies);

                        // Allow page and set updated cookies
                        return Ok::<_, Rejection>(response);
                    }
                }

                // Allow without updating cookies
                Ok::<_, Rejection>(reply.into_response())
            },
        ))
}

fn cookie_map(tokens: Option<(String, String)>) -> Option<warp::http::header::HeaderMap> {
    let tokens = tokens?;

    let mut cookies = warp::http::header::HeaderMap::new();
    cookies.append(
        "Set-Cookie",
        format!(
            "auth_token={}; HttpOnly; Secure; SameSite=Strict; Path=/",
            tokens.0
        )
        .parse()
        .unwrap(),
    );
    cookies.append(
        "Set-Cookie",
        format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Path=/",
            tokens.1
        )
        .parse()
        .unwrap(),
    );

    Some(cookies)
}
