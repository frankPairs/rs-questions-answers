mod crypt;
mod handlers;
mod profanity;
mod store;
mod types;

use dotenv;
use handle_errors::error_handler;
use std::env;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, path, Filter};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let log_filter = env::var("RUST_LOG").unwrap_or("questions_answers".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable is missing.");
    let port = env::var("PORT")
        .unwrap_or(String::from("8000"))
        .parse::<u16>()
        .unwrap_or(8000);

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let store = store::Store::new(&database_url).await;
    let store_filter = warp::any().map(move || store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("Content-Type")
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE]);
    // Questions Handlers
    let get_questions = warp::get()
        .and(path("questions"))
        .and(path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(handlers::questions::get_questions_handler);
    let add_question = warp::post()
        .and(path("questions"))
        .and(path::end())
        .and(handlers::auth::auth())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::questions::add_question_handler);
    let update_question = warp::put()
        .and(path("questions"))
        .and(path::param::<i32>())
        .and(path::end())
        .and(handlers::auth::auth())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::questions::update_question_handler);
    let delete_question = warp::delete()
        .and(path("questions"))
        .and(path::param::<i32>())
        .and(path::end())
        .and(handlers::auth::auth())
        .and(store_filter.clone())
        .and_then(handlers::questions::delete_question_handler);
    let get_question = warp::get()
        .and(path("questions"))
        .and(path::param::<i32>())
        .and(path::end())
        .and(store_filter.clone())
        .and_then(handlers::questions::get_question_handler);
    // Answers Handlers
    let add_answer = warp::post()
        .and(path("answers"))
        .and(path::end())
        .and(handlers::auth::auth())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::answers::add_answer_handler);
    let registration = warp::post()
        .and(path("registration"))
        .and(path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::auth::register);
    let login = warp::post()
        .and(path("login"))
        .and(path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::auth::login);

    // Global Routes
    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(get_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(error_handler);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
