pub mod blog;
pub mod cookies;

#[derive(Debug)]
pub struct Unauthorized;
impl warp::reject::Reject for Unauthorized {}

#[derive(Debug)]
pub struct Authorized;
impl warp::reject::Reject for Authorized {}
