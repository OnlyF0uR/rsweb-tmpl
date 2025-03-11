use dotenvy::dotenv;
use warp::{Filter, reject::Rejection, reply::Reply};

const PORT: u16 = 3030;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Serve static files (like router.js)
    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    // Frontend application routes
    let app_routes = static_files
        .or(rsweb_app::routes::authenticated())
        .or(rsweb_app::routes::explore())
        .or(rsweb_app::routes::blog())
        .or(rsweb_app::routes::login())
        .or(rsweb_app::routes::root());

    // API routes
    let api_routes = rsweb_api::routes::login().or(rsweb_api::routes::register());

    // Combine routes
    let routes = app_routes.or(api_routes).recover(handle_rejection);

    // Start server
    println!("Listening on https://localhost:{}", PORT);
    warp::serve(routes)
        .tls()
        .cert_path("cert.pem")
        .key_path("cert.key")
        .run(([127, 0, 0, 1], PORT))
        .await;
}

async fn handle_rejection(
    err: Rejection,
) -> Result<Box<dyn Reply + Send>, std::convert::Infallible> {
    let mut r: Box<dyn Reply + Send> = Box::new(warp::reply::with_status(
        "Internal server error",
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ));

    // General
    if err.is_not_found() {
        r = Box::new(warp::reply::with_status(
            "Not found",
            warp::http::StatusCode::NOT_FOUND,
        ));
    }

    // Specifics
    // App
    if err.find::<rsweb_app::filters::Unauthorized>().is_some() {
        // FIX: This redirect does not seem to actually do anything
        r = Box::new(warp::redirect::see_other(warp::http::Uri::from_static(
            "/login",
        )));
    } else if err.find::<rsweb_app::filters::Authorized>().is_some() {
        // FIX: This redirect does not seem to actually do anything
        r = Box::new(warp::redirect::see_other(warp::http::Uri::from_static("/")));
    }
    // Api
    if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
        || err.find::<warp::reject::LengthRequired>().is_some()
        || err.find::<warp::reject::PayloadTooLarge>().is_some()
        || err.find::<rsweb_api::filters::BadRequest>().is_some()
    {
        r = Box::new(warp::reply::with_status(
            "Invalid request body",
            warp::http::StatusCode::BAD_REQUEST,
        ));
    } else if err.find::<rsweb_api::filters::Unauthorized>().is_some() {
        r = Box::new(warp::reply::with_status(
            "Unauthorized",
            warp::http::StatusCode::UNAUTHORIZED,
        ));
    }

    Ok(r)
}
