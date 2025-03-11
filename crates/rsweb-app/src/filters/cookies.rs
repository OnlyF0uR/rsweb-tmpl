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

pub fn without_auth() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::cookie::optional("auth_token"))
        .and(warp::cookie::optional("refresh_token"))
        .and_then(
            |auth_token: Option<String>, refresh_token: Option<String>| async move {
                match Claims::try_from_tokens(&auth_token, &refresh_token).await {
                    Ok((_claims, _updated_tokens)) => Err(warp::reject::custom(super::Authorized)),
                    Err(_) => Ok(()),
                }
            },
        )
        .untuple_one()
}

pub fn with_opt_auth()
-> impl Filter<Extract = (Option<AuthSession>,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::cookie::optional("auth_token"))
        .and(warp::cookie::optional("refresh_token"))
        .and_then(
            |auth_token: Option<String>, refresh_token: Option<String>| async move {
                let (claims, updated_tokens) =
                    match Claims::try_from_tokens(&auth_token, &refresh_token).await {
                        Ok((claims, updated_tokens)) => (claims, updated_tokens),
                        Err(_) => return Ok(None),
                    };

                Ok::<_, warp::reject::Rejection>(Some(AuthSession {
                    claims,
                    updated_tokens,
                }))
            },
        )
}
