use sqlx::{query, query_as, SqlitePool};

use crate::User;

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = SqlitePool::connect("sqlite://mcss.sqlite?mode=rwc").await?;
        sqlx::migrate!("../migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn add_user(&self, user: &User) -> anyhow::Result<()> {
        let id = user.id.clone();
        let username = user.username.clone();
        let password_hash = user.password_hash.clone();
        let avatar = user.avatar_image.clone();
        query!(
            "INSERT INTO users (id, username, password_hash, avatar_image) VALUES (?1, ?2, ?3, ?4)",
            id,
            username,
            password_hash,
            avatar
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        let result = query_as!(User,
            "SELECT id, username, password_hash, avatar_image, selected_skin_id, selected_cape_id FROM users"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }
}
