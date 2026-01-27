#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Config {
            database_url: "postgresql://postgres:password@localhost:5432/pgmq".to_string(),
        }
    }

    pub fn from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in environment variables");

        Config { database_url }
    }
}
