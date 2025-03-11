use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub struct DB {
    pub pool: PgPool,
}

impl DB {
    // Initialize the connection pool asynchronously
    async fn initialize() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPool::connect(&database_url).await?;
        Ok(DB { pool })
    }

    pub async fn from_url(
        database_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pool = PgPool::connect(database_url).await?;
        Ok(DB { pool })
    }
}

// Global database connection pool (wrapped in Arc for concurrency)
static DB_INSTANCE: OnceCell<Arc<DB>> = OnceCell::const_new();

// Function to get a reference to the initialized DB
pub async fn get_db() -> Arc<DB> {
    DB_INSTANCE
        .get_or_init(|| async {
            let db = DB::initialize()
                .await
                .expect("Failed to initialize database");
            Arc::new(db)
        })
        .await
        .clone()
}

// Re-export individual modules
pub mod user;
