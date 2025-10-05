use std::sync::Arc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug,Deserialize, Serialize,ToSchema)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub password_hash: String,
    pub avatar_image: Option<Vec<u8>>,
    pub selected_skin_id: Option<String>,
    pub selected_cape_id: Option<String>,
}

mod db;
mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Initialize database and add a user for demonstration
    let db = db::Db::new().await?;

    // Pass the database path as shared state instead of the connection
    let shared_state = Arc::new(db);

    // build our application with a route
    api::handle_web(shared_state).await?;
    Ok(())
}
