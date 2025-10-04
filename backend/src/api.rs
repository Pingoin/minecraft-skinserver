use std::sync::Arc;
use tokio::net::TcpListener;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use anyhow::Error;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::User;
use crate::db::Db;

pub(crate) async fn handle_web(state: Arc<Db>) -> anyhow::Result<()> {
    let (router, api) = OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(root_handler))
        .split_for_parts();

    let router = router
        .with_state(state)
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

    let listener = TcpListener::bind(("0.0.0.0", 8080)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/user",
    responses(
        (status = 200, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_user(
    State(db): State<Arc<Db>>,
    Json(payload): Json<User>,
) -> Result<(StatusCode, Json<User>), AppError> {

    // Optionally, add the user to the database here
    db.add_user(&payload).await?;

    Ok((StatusCode::CREATED, Json(payload)))
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "Root endpoint", body = [User]),
        (status = 500, description = "Internal server error")
    )
)]
async fn root_handler(State(db): State<Arc<Db>>) -> Result<Json<Vec<User>>, AppError> {
    // Optionally, retrieve user data from the database here
    let users = db.get_users().await?;
    Ok(Json(users))
}

#[derive(Debug)]
pub struct AppError(pub Error);

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        AppError(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Du kannst hier Logging einbauen oder Error-Types unterscheiden
        eprintln!("Fehler: {:?}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
