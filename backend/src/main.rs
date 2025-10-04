use std::sync::Arc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug,Deserialize, Serialize,ToSchema)]
pub struct User {
   pub id: i32,
   pub name: String,
   pub password: String,
   pub data: Option<Vec<u8>>,
}

mod db;
mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Initialize database and add a user for demonstration
    let db_path = "data.sqlite";
    let db = db::Db::new(db_path)?;
    db.init().await?;

    // Pass the database path as shared state instead of the connection
    let shared_state = Arc::new(db);

    // build our application with a route
    api::handle_web(shared_state).await?;
    Ok(())
}
