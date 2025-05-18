use axum::{Json, Router, routing::post};
use runner_schema::web::{RunnerRequest, RunnerResponse};

#[tokio::main]
#[allow(clippy::unwrap_used)]
async fn main() {
    env_logger::init();

    log::info!("Starting runner...");

    let app = Router::new().route("/run", post(router_run));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn router_run(Json(payload): Json<RunnerRequest>) -> Json<RunnerResponse> {
    runner::run(payload).map(Json).unwrap_or_else(|err| {
        log::error!("Internal Error: {}", err);
        Json(RunnerResponse {
            state: runner_schema::state::RunnerState::InternalError,
        })
    })
}
