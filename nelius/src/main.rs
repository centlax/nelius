use axum::{routing::get, Router};
mod parser;
mod query;

// Each route provide either
#[allow(dead_code)]
enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PATCH,
    DELETE,
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let router = Router::new().route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
