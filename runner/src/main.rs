use axum::{Json, Router, extract::State, routing::post};
use envman::EnvMan;
use runner::env::RunnerOption;
use runner_schema::web::{RunnerRequest, RunnerResponse};

struct RunnerState {
    pub option: RunnerOption,
    pub runners: runner::runner::Runners,
}

#[tokio::main]
#[allow(clippy::unwrap_used, clippy::expect_used)]
async fn main() {
    env_logger::init();

    log::info!("Starting runner...");

    let env = RunnerOption::load().expect("Failed to load environment variables");

    log::info!("Runner environment: {:#?}", env);

    let state = RunnerState {
        option: env,
        runners: runner::runner::Runners::new(),
    };

    static STATE: std::sync::OnceLock<RunnerState> = std::sync::OnceLock::new();

    let state: &'static RunnerState = STATE.get_or_init(|| state);

    let app = Router::new()
        .route("/run", post(router_run))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn router_run(
    State(state): State<&'static RunnerState>,
    Json(payload): Json<RunnerRequest>,
) -> Json<RunnerResponse> {
    runner::run(&state.runners, payload, &state.option)
        .map(Json)
        .unwrap_or_else(|err| {
            log::error!("Internal Error: {}", err);
            Json(RunnerResponse {
                state: runner_schema::state::RunnerState::InternalError,
            })
        })
}

#[allow(clippy::expect_used)]
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            log::info!("Runner shutdown signal received");
        },
        _ = terminate => {
            log::info!("Runner terminate signal received");
        },
    }
}
