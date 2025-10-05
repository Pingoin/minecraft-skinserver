use std::sync::Arc;

use base64::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
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

#[derive(Debug,Deserialize, Serialize,ToSchema,TS)]
#[ts(export)]
pub enum SkinType {
    Skin,
    Cape,
    Elytra,
}

#[derive(Debug,Deserialize, Serialize,ToSchema)]
pub struct Texture {
    pub id: Option<String>,
    pub skin_name: String,
    pub texture_type: SkinType,
    pub image_data: Vec<u8>,
}

#[derive(Debug,Deserialize, Serialize,ToSchema,TS)]
#[ts(export)]
pub struct ApiTexture {
    pub id: Option<String>,
    pub skin_name: String,
    pub texture_type: SkinType,
    pub image_data: String, // Base64 encoded
}   

impl From<Texture> for ApiTexture {
    fn from(texture: Texture) -> Self {
        ApiTexture {
            id: texture.id,
            skin_name: texture.skin_name,
            texture_type: texture.texture_type,
            image_data:BASE64_STANDARD.encode(&texture.image_data),
        }
    }
}
impl From<ApiTexture> for Texture {
    fn from(api_texture: ApiTexture) -> Self {
        Texture {
            id: api_texture.id,
            skin_name: api_texture.skin_name,
            texture_type: api_texture.texture_type,
            image_data: BASE64_STANDARD.decode(&api_texture.image_data).unwrap_or_default(),
        }
    }
}