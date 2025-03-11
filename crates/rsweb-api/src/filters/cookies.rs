use rsweb_auth::claims::{AuthSession, Claims};
use warp::Filter;

pub fn with_auth() -> impl Filter<Extract = (AuthSession,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::cookie::optional("auth_token"))
        .and(warp::cookie::optional("refresh_token"))
        .and_then(
            |auth_token: Option<String>, refresh_token: Option<String>| async move {
                let (claims, updated_tokens) =
                    match Claims::try_from_tokens(&auth_token, &refresh_token).await {
                        Ok((claims, updated_tokens)) => (claims, updated_tokens),
                        Err(_) => return Err(warp::reject::custom(super::Unauthorized)),
                    };

                Ok::<_, warp::reject::Rejection>(AuthSession {
                    claims,
                    updated_tokens,
                })
            },
        )
}

pub fn with_creator_auth() -> impl Filter<Extract = (AuthSession,), Error = warp::Rejection> + Clone
{
    warp::any()
        .and(warp::cookie::optional("auth_token"))
        .and(warp::cookie::optional("refresh_token"))
        .and_then(
            |auth_token: Option<String>, refresh_token: Option<String>| async move {
                let (claims, updated_tokens) =
                    match Claims::try_from_tokens(&auth_token, &refresh_token).await {
                        Ok((claims, updated_tokens)) => (claims, updated_tokens),
                        Err(_) => return Err(warp::reject::custom(super::Unauthorized)),
                    };

                if !claims.has_creator_privilege() {
                    return Err(warp::reject::custom(super::Unauthorized));
                }

                Ok::<_, warp::reject::Rejection>(AuthSession {
                    claims,
                    updated_tokens,
                })
            },
        )
}

pub fn with_auth_no_profile()
-> impl Filter<Extract = (AuthSession,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::cookie::optional("auth_token"))
        .and(warp::cookie::optional("refresh_token"))
        .and_then(
            |auth_token: Option<String>, refresh_token: Option<String>| async move {
                let (claims, updated_tokens) =
                    match Claims::try_from_tokens(&auth_token, &refresh_token).await {
                        Ok((claims, updated_tokens)) => (claims, updated_tokens),
                        Err(_) => return Err(warp::reject::custom(super::Unauthorized)),
                    };

                Ok::<_, warp::reject::Rejection>(AuthSession {
                    claims,
                    updated_tokens,
                })
            },
        )
}
