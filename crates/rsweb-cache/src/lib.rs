use deadpool_redis::{Config, Connection, Pool, Runtime};

use std::env;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub struct Cache {
    pub pool: Pool,
}

impl Cache {
    // Initialize the connection pool asynchronously
    async fn initialize() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

        let cfg = Config::from_url(redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

        Ok(Cache { pool })
    }
}

// Global database connection pool (wrapped in Arc for concurrency)
static CACHE_INSTANCE: OnceCell<Arc<Cache>> = OnceCell::const_new();

// Function to get a reference to the initialized DB
pub async fn get_cache() -> Arc<Cache> {
    CACHE_INSTANCE
        .get_or_init(|| async {
            let cache = Cache::initialize()
                .await
                .expect("Failed to initialize cache");
            Arc::new(cache)
        })
        .await
        .clone()
}

pub async fn get_connection() -> Connection {
    let cache = get_cache().await;
    cache.pool.get().await.unwrap()
}
