use parking_lot::RwLock;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};

const DB_MAX_CONNECTIONS: u32 = 5;

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(DB_MAX_CONNECTIONS)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(_) => panic!("Could not establsh a database connection"),
        };

        Store {
            connection: db_pool,
        }
    }
}
