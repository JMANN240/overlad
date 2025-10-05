use overlad_api::{Image, User};
use sqlx::{SqlitePool, query_as};

use crate::{db::user::DbUser, util::to_row_not_found};

pub struct DbImage {
    pub id: String,
    pub user_id: i64,
    pub extension: String,
}

impl DbImage {
    pub async fn insert(
        pool: &SqlitePool,
        id: &str,
        user_id: i64,
        extension: &str,
    ) -> sqlx::Result<DbImage> {
        query_as!(Self, "INSERT INTO images VALUES (?, ?, ?) RETURNING *", id, user_id, extension)
            .fetch_one(pool)
            .await
    }

    pub async fn get_all(pool: &SqlitePool) -> sqlx::Result<Vec<Self>> {
        query_as!(Self, "SELECT * FROM images")
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: impl AsRef<str>) -> sqlx::Result<Option<Self>> {
        let id_ref = id.as_ref();

        query_as!(Self, "SELECT * FROM images WHERE id = ?", id_ref)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_by_user_id(pool: &SqlitePool, user_id: i64) -> sqlx::Result<Vec<Self>> {
        query_as!(Self, "SELECT * FROM images WHERE user_id = ?", user_id,)
            .fetch_all(pool)
            .await
    }

    pub async fn get_db_user(&self, pool: &SqlitePool) -> sqlx::Result<DbUser> {
        DbUser::get_by_id(pool, self.user_id)
            .await
            .and_then(to_row_not_found)
    }

    pub async fn get_user(&self, pool: &SqlitePool) -> sqlx::Result<User> {
        self.get_db_user(pool).await.map(User::from)
    }

    pub async fn into_image(self, pool: &SqlitePool) -> sqlx::Result<Image> {
        let user = self.get_user(pool).await?;

        Ok(Image {
            id: self.id,
            user,
            extension: self.extension,
        })
    }
}
