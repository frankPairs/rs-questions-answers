mod handlers;
mod store;
mod types;

use handle_errors::error_handler;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, path, Filter};

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "questions_answers".to_string());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let store = store::Store::new();
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
        .and_then(handlers::questions::get_questions_handler)
        .with(warp::trace(
            |info| tracing::info_span!("get questions request", method = %info.method(), path = %info.path(), id = %uuid::Uuid::new_v4()),
        ));
    let add_question = warp::post()
        .and(path("questions"))
        .and(path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::questions::add_question_handler);
    let update_question = warp::put()
        .and(path("questions"))
        .and(path::param::<String>())
        .and(path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::questions::update_question_handler);
    let delete_question = warp::delete()
        .and(path("questions"))
        .and(path::param::<String>())
        .and(path::end())
        .and(store_filter.clone())
        .and_then(handlers::questions::delete_question_handler);
    let get_question = warp::delete()
        .and(path("questions"))
        .and(path::param::<String>())
        .and(path::end())
        .and(store_filter.clone())
        .and_then(handlers::questions::get_question_handler);
    // Answers Handlers
    let add_answer = warp::post()
        .and(path("answers"))
        .and(path::end())
        .and(warp::body::form())
        .and(store_filter.clone())
        .and_then(handlers::answers::add_answer_handler);

    // Global Routes
    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(get_question)
        .or(add_answer)
        .with(cors)
        .with(warp::trace::request())
        .recover(error_handler);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
