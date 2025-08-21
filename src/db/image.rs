use sqlx::{SqlitePool, query, query_as, sqlite::SqliteQueryResult};

pub struct Image {
    pub id: String,
    pub user_id: i64,
}

impl Image {
    pub async fn insert(
        pool: &SqlitePool,
        id: &str,
        user_id: i64,
    ) -> sqlx::Result<SqliteQueryResult> {
        query!("INSERT INTO images VALUES (?, ?)", id, user_id,)
            .execute(pool)
            .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> sqlx::Result<Image> {
        query_as!(Image, "SELECT * FROM images WHERE id = ?", id,)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_user_id(pool: &SqlitePool, user_id: i64) -> sqlx::Result<Vec<Image>> {
        query_as!(Image, "SELECT * FROM images WHERE user_id = ?", user_id,)
            .fetch_all(pool)
            .await
    }
}
