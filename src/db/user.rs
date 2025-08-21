use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use sqlx::{SqlitePool, query, query_as, sqlite::SqliteQueryResult};

pub struct User {
    pub id: i64,
    pub username: String,
    pub passhash: String,
    pub salt: String,
}

impl User {
    pub async fn insert(
        pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> sqlx::Result<SqliteQueryResult> {
        let salt = SaltString::generate(&mut OsRng);

        let passhash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let salt_string = salt.to_string();

        query!(
            "INSERT INTO users (username, passhash, salt) VALUES (?, ?, ?)",
            username,
            passhash,
            salt_string,
        )
        .execute(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<User> {
        query_as!(User, "SELECT * FROM users WHERE id = ?", id,)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_username(pool: &SqlitePool, username: &str) -> sqlx::Result<Option<User>> {
        query_as!(User, "SELECT * FROM users WHERE username = ?", username,)
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
