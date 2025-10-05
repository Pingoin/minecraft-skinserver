use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use anyhow::Error;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{ApiTexture, User};
use crate::db::Db;

pub(crate) async fn handle_web(state: Arc<Db>) -> anyhow::Result<()> {

        let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "http://127.0.0.1:5173".parse().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    let (router, api) = OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(get_users))
        .routes(routes!(create_texture))
        .routes(routes!(get_textures))
        .split_for_parts();

    let router = router
        .with_state(state)
        .layer(cors)
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
    let user=db.add_user(payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "Root endpoint", body = [User]),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_users(State(db): State<Arc<Db>>) -> Result<Json<Vec<User>>, AppError> {
    // Optionally, retrieve user data from the database here
    let users = db.get_users().await?;
    Ok(Json(users))
}

#[utoipa::path(
    post,
    path = "/api/texture",
    responses(
        (status = 200, description = "Texture created successfully", body = ApiTexture),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_texture(
    State(db): State<Arc<Db>>,
    Json(payload): Json<ApiTexture>,
) -> Result<(StatusCode, Json<ApiTexture>), AppError> {

    // Optionally, add the texture to the database here
    let texture=db.add_texture(payload.into()).await?;

    Ok((StatusCode::CREATED, Json(texture.into())))
}

#[utoipa::path(
    get,
    path = "/api/textures",
    responses(
        (status = 200, description = "Root endpoint", body = [ApiTexture]),
        (status = 500, description = "Internal server error")
    )
)]
async fn get_textures(State(db): State<Arc<Db>>) -> Result<Json<Vec<ApiTexture>>, AppError> {
    // Optionally, retrieve user data from the database here
    let textures = db.get_textures().await?;
    Ok(Json(textures.into_iter().map(ApiTexture::from).collect()))
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
