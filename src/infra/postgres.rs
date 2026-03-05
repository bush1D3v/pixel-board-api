use deadpool_postgres::{Config, Pool, PoolConfig, Runtime};
use std::env;
use tokio_postgres::NoTls;

pub struct Postgres;

impl Postgres {
    pub fn pool() -> Pool {
        let mut cfg = Config::new();
        cfg.host = Some(env::var("DB_HOST").expect("DB_HOST must be set"));
        cfg.port = Some(
            env::var("DB_PORT")
                .expect("DB_PORT must be set")
                .parse()
                .expect("DB_PORT must be a valid number"),
        );
        cfg.dbname = Some(env::var("DB_NAME").expect("DB_NAME must be set"));
        cfg.user = Some(env::var("DB_USER").expect("DB_USER must be set"));
        cfg.password = Some(env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"));
        cfg.pool = PoolConfig::new(
            env::var("DB_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse::<usize>()
                .expect("DB_POOL_SIZE must be a valid number"),
        )
        .into();

        cfg.create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Failed to create Postgres pool")
    }
}
