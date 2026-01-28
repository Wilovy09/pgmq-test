/// Verify if pgmq extension is available
///
/// If not, create it
///
/// You can see https://github.com/pgmq/pgmq/blob/main/INSTALLATION.md
pub async fn verify_pgmq_extension(client: &sqlx::PgPool) {
    match sqlx::query!("SELECT * FROM pg_available_extensions WHERE NAME = 'pgmq';")
        .fetch_one(client)
        .await
    {
        Ok(record) => {
            println!("Verified pgmq extension {:?}", record);
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
    match sqlx::query!("CREATE EXTENSION IF NOT EXISTS pgmq;")
        .execute(client)
        .await
    {
        Ok(_) => {
            println!("pgmq extension created or already exists");
        }
        Err(e) => {
            eprintln!("Failed to create pgmq extension: {e}");
        }
    }
}
