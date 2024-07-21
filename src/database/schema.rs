use anyhow::{bail, Result};
use sqlx::{Acquire, Executor, PgPool};
use tracing::info;

pub const MIGRATIONS: [&str; 2] = [
    include_str!("migrations/0001-init.sql"),
    include_str!("migrations/0002-basic.sql"),
];
pub const CURRENT_VERSION: u32 = MIGRATIONS.len() as u32;

pub async fn update(db: &PgPool) -> Result<()> {
    let dbname = sqlx::query_scalar::<_, String>("SELECT current_database()")
        .fetch_one(db)
        .await?;
    let lock = crc::Crc::<u64>::new(&crc::CRC_64_GO_ISO)
        .checksum(format!("odino-update-{}", dbname).as_bytes())
        as i64;
    let mut conn = db.acquire().await?;
    sqlx::query("SELECT pg_advisory_lock($1)")
        .bind(lock)
        .execute(&mut *conn)
        .await?;

    let mut version = get_version(db).await?;
    info!(
        current = version,
        target = CURRENT_VERSION,
        "Updating database schema"
    );
    while version != CURRENT_VERSION {
        info!(version, "Migrating database schema");
        let mut transaction = conn.begin().await?;

        (*transaction).execute(MIGRATIONS[version as usize]).await?;
        version += 1;

        sqlx::query("UPDATE schema_version SET \"version\" = $1")
            .bind(version as i32)
            .execute(&mut *transaction)
            .await?;

        info!(version, "Committing schema changes");
        transaction.commit().await?;
    }

    sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(lock)
        .execute(&mut *conn)
        .await?;
    info!("Finished database schema update");
    Ok(())
}

pub async fn get_version(db: &PgPool) -> Result<u32> {
    match sqlx::query_scalar::<_, i32>(
        "SELECT version FROM schema_version WHERE id = 0 LIMIT 1",
    )
    .fetch_optional(db)
    .await
    .unwrap_or(None)
    {
        Some(version) => Ok(version.try_into().unwrap()),
        None => Ok(0),
    }
}

pub async fn check(db: &PgPool) -> Result<()> {
    let schema_version = get_version(db).await?;
    if schema_version < CURRENT_VERSION {
        bail!(
            r"Current database schema is outdated, this odino expects {}, but the database is {}.
                    Run odino with --update to update the schema.",
            CURRENT_VERSION,
            schema_version
        )
    } else if schema_version > CURRENT_VERSION {
        bail!(
            "This version of odino is outdated. The database schema is {}, but this version of odino supports {}.",
            schema_version,
            CURRENT_VERSION
        );
    }
    Ok(())
}
