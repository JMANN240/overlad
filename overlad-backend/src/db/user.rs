use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use overlad_api::User;
use sqlx::{SqlitePool, query_as};

pub struct DbUser {
    pub id: i64,
    pub username: String,
    pub passhash: String,
}

impl DbUser {
    pub async fn insert(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> sqlx::Result<Self> {
        let salt = SaltString::generate(&mut OsRng);

        let passhash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        query_as!(
            Self,
            "INSERT INTO users (username, passhash) VALUES (?, ?) RETURNING *",
            username,
            passhash,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Option<Self>> {
        query_as!(Self, "SELECT * FROM users WHERE id = ?", id,)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_by_username(pool: &SqlitePool, username: &str) -> sqlx::Result<Option<Self>> {
        query_as!(Self, "SELECT * FROM users WHERE username = ?", username,)
            .fetch_optional(pool)
            .await
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let passhash = PasswordHash::new(&self.passhash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &passhash)
            .is_ok()
    }
}

impl From<DbUser> for User {
    fn from(value: DbUser) -> Self {
        User {
            id: value.id,
            username: value.username,
        }
    }
}
