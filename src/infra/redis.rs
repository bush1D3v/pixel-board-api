use deadpool_redis::{
    redis::{cmd, RedisResult},
    Config, ConnectionAddr, ConnectionInfo, Pool, PoolConfig, RedisConnectionInfo, Runtime,
    Timeouts,
};
use std::{env, time::Duration};

pub struct Redis;

impl Redis {
    pub async fn set(pool: &Pool, key: &str, value: &str) -> RedisResult<()> {
        let mut conn = pool.get().await.unwrap();
        cmd("SET")
            .arg(&[key, value])
            .query_async::<()>(&mut conn)
            .await
    }

    pub async fn get(pool: &Pool, key: &str) -> RedisResult<String> {
        let mut conn = pool.get().await.unwrap();
        cmd("GET")
            .arg(&[key])
            .query_async::<String>(&mut conn)
            .await
    }

    pub async fn delete(pool: &Pool, key: &str) -> RedisResult<i32> {
        let mut conn = pool.get().await.unwrap();
        cmd("DEL")
            .arg(&[key])
            .query_async::<i32>(&mut conn)
            .await
    }

    pub async fn set_ex(pool: &Pool, key: &str, value: &str, seconds: u64) -> RedisResult<()> {
        let mut conn = pool.get().await.unwrap();
        cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(seconds)
            .query_async::<()>(&mut conn)
            .await
    }

    pub async fn pool() -> Pool {
        let mut cfg = Config::default();
        cfg.connection = Some(ConnectionInfo {
            addr: ConnectionAddr::Tcp(
                env::var("REDIS_HOST").expect("REDIS_HOST must be set"),
                env::var("REDIS_PORT")
                    .expect("REDIS_PORT must be set")
                    .parse()
                    .expect("REDIS_PORT must be a valid number"),
            ),
            redis: RedisConnectionInfo {
                db: env::var("REDIS_DB")
                    .unwrap_or_else(|_| "0".to_string())
                    .parse()
                    .expect("REDIS_DB must be a valid number"),
                username: env::var("REDIS_USER").ok(),
                password: env::var("REDIS_PASSWORD").ok(),
                protocol: Default::default(),
            },
        });
        cfg.pool = Some(PoolConfig {
            max_size: env::var("REDIS_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .expect("REDIS_POOL_SIZE must be a valid number"),
            timeouts: Timeouts {
                wait: Some(Duration::from_secs(30)),
                create: Some(Duration::from_secs(30)),
                recycle: Some(Duration::from_secs(30)),
            },
            queue_mode: Default::default(),
        });

        cfg.create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create Redis pool")
    }
}
