use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Params;

use crate::User;

pub struct Db {
    pool: Pool<SqliteConnectionManager>,
}

impl Db {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        Ok(Self { pool })
    }

    async fn execute<P>(&self, sql: String, params: P) -> anyhow::Result<()>
    where
        P: Params + Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(anyhow::Error::from)?;
            conn.execute(sql.as_str(), params)
                .map_err(anyhow::Error::from)
        })
        .await??;
        Ok(())
    }

    pub async fn init(&self) -> anyhow::Result<()> {
        self.execute(
            "CREATE TABLE IF NOT EXISTS person (
                    id   INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    password TEXT NOT NULL,
                    data BLOB
                )".to_string(),
            (), // empty list of parameters.
        )
        .await?;
        Ok(())
    }

    pub async fn add_user(&self, user: &User) -> anyhow::Result<()> {
        let name = user.name.clone();
        let password = user.password.clone();
        let data = user.data.clone();
        self.execute(
            "INSERT INTO person (name, password, data) VALUES (?1, ?2, ?3)".to_string(),
            (name, password, data),
        )
        .await?;
        Ok(())
    }

    pub async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        self.query(
            "SELECT id, name, password, data FROM person".to_string(),
            |row| User {
                id: row.get(0).unwrap_or(0),
                name: row.get(1).unwrap_or_default(),
                password: row.get(2).unwrap_or_default(),
                data: row.get(3).unwrap_or_default(),
            },
        ).await
    }

    async fn query<F, T>(&self, sql: String, map_function: F) -> anyhow::Result<Vec<T>>
    where
        F: Fn(&rusqlite::Row) -> T+ Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        let results = tokio::task::spawn_blocking(move || {
            let conn = pool.get().map_err(anyhow::Error::from)?;
            let mut stmt = conn.prepare(sql.as_str()).map_err(anyhow::Error::from)?;
            let response_iter = stmt
                .query_map([], |row| {
                    Ok(map_function(row))
                })
                .map_err(anyhow::Error::from)?;

            let mut results = Vec::new();   
            for result in response_iter {
                results.push(result.map_err(anyhow::Error::from)?);
            }
        Ok(results)
        })
        .await?;
        results
    }

}
