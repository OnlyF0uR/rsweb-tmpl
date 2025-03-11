use warp::Filter;

use crate::endpoints::{signin, signup};

pub fn login() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "login")
        .and(warp::post())
        .and(signin::filter())
        .and_then(signin::handle)
}

pub fn register() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("api" / "register")
        .and(warp::post())
        .and(signup::filter())
        .and_then(signup::handle)
}
