use super::get_db;
use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct UserEssentials {
    pub id: i32,
    pub email: String,
    pub handle: String,
    pub role: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserDetails {
    pub id: i32,
    pub email: String,
    pub password: Option<String>,
    pub password_salt: Option<String>,
    pub handle: String,
    pub role: String,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub banned_at: Option<PrimitiveDateTime>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GoogleUserDetails {
    pub id: i32,
    pub email: String,
    pub handle: String,
    pub role: String,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub banned_at: Option<PrimitiveDateTime>,
}

pub struct UserService;

impl UserService {
    pub async fn insert_user_email(
        email: &str,
        password: &str,
        salt: &str,
        username: &str,
    ) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query!(
            "INSERT INTO users (email, password, password_salt, handle) VALUES ($1, $2, $3, $4) RETURNING id",
            email,
            password,
            salt,
            username
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(result.id)
    }

    pub async fn insert_user_google(
        google_sub: &str,
        email: &str,
        username: &str,
    ) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query!(
            "INSERT INTO users (google_sub, email, handle) VALUES ($1, $2, $3) RETURNING id",
            google_sub,
            email,
            username
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(result.id)
    }

    pub async fn get_user_details(
        email: &str,
    ) -> Result<UserDetails, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query_as!(
            UserDetails,
            "SELECT u.id, u.email, u.password, u.password_salt, u.handle, u.role, u.banned, u.banned_at, u.ban_reason FROM users u WHERE u.email = $1",
            email
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_google_user_details(
        sub: &str,
    ) -> Result<GoogleUserDetails, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query_as!(
            GoogleUserDetails,
            "SELECT u.id, u.email, u.handle, u.role, u.banned, u.banned_at, u.ban_reason FROM users u WHERE u.google_sub = $1",
            sub
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_user_essentials_by_refresh_token(
        refresh_token: &str,
    ) -> Result<UserEssentials, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query_as!(
            UserEssentials,
            "SELECT u.id, u.email, u.handle, u.role FROM users u JOIN refresh_tokens r ON u.id = r.user_id WHERE r.token = $1",
            refresh_token
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(result)
    }

    pub async fn insert_user_refresh_token(
        user_id: i32,
        token: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        sqlx::query!(
            "INSERT INTO refresh_tokens (user_id, token) VALUES ($1, $2)",
            user_id,
            token
        )
        .execute(&db.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_user_refresh_token(
        user_id: i32,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
            .execute(&db.pool)
            .await?;

        Ok(result.rows_affected())
    }

    #[allow(dead_code)]
    pub async fn handle_exists(
        handle: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE handle = $1) AS exists",
            handle
        )
        .fetch_one(&db.pool)
        .await?;

        let exists = matches!(result.exists, Some(true));
        Ok(exists)
    }

    #[allow(dead_code)]
    pub async fn update_user_banned_status(
        user_id: i32,
        banned: bool,
        reason: Option<&str>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query!(
            "UPDATE users SET banned = $1, ban_reason = $2, banned_at = CURRENT_TIMESTAMP WHERE id = $3",
            banned,
            reason,
            user_id
        )
        .execute(&db.pool)
        .await?;

        Ok(result.rows_affected())
    }

    #[allow(dead_code)]
    pub async fn delete_user(
        user_id: i32,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let db = get_db().await;
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&db.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
