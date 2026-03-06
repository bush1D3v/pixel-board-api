use deadpool_postgres::Pool;

type DynError = Box<dyn std::error::Error + Send + Sync>;

struct Migration {
    version: i32,
    name: &'static str,
    sql: &'static str,
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "create_users",
        sql: include_str!("../../migrations/001_create_users/up.sql"),
    },
    Migration {
        version: 2,
        name: "create_blocks",
        sql: include_str!("../../migrations/002_create_blocks/up.sql"),
    },
    Migration {
        version: 3,
        name: "create_reservations",
        sql: include_str!("../../migrations/003_create_reservations/up.sql"),
    },
];

pub async fn run_migrations(pool: &Pool) -> Result<(), DynError> {
    let mut client = pool.get().await?;

    client
        .batch_execute(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version     INT PRIMARY KEY,
                name        TEXT NOT NULL,
                applied_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
            "#,
        )
        .await?;

    for migration in MIGRATIONS {
        let applied = client
            .query_opt(
                "SELECT 1 FROM schema_migrations WHERE version = $1",
                &[&migration.version],
            )
            .await?
            .is_some();

        if applied {
            continue;
        }

        log::info!(
            "Applying migration v{} ({})",
            migration.version,
            migration.name
        );

        let tx = client.transaction().await?;
        tx.batch_execute(migration.sql).await?;
        tx.execute(
            "INSERT INTO schema_migrations (version, name) VALUES ($1, $2)",
            &[&migration.version, &migration.name],
        )
        .await?;
        tx.commit().await?;

        log::info!(
            "Migration applied: v{} ({})",
            migration.version,
            migration.name
        );
    }

    Ok(())
}
