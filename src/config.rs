#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub secret_key: String,
    pub frontend_url: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_email: String,
    pub smtp_from_name: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Config {
            database_url: "postgresql://postgres:password@localhost:5432/pgmq".to_string(),
            secret_key: "supersecretkey".to_string(),
            frontend_url: "http://localhost:3000".to_string(),
            smtp_host: "smtp.example.com".to_string(),
            smtp_port: 587,
            smtp_username: "smtp_user".to_string(),
            smtp_password: "smtp_password".to_string(),
            smtp_from_email: "no-reply@example.com".to_string(),
            smtp_from_name: "Example".to_string(),
        }
    }

    pub fn from_env() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in environment variables");
        let secret_key =
            std::env::var("SECRET_KEY").expect("SECRET_KEY must be set in environment variables");
        let frontend_url = std::env::var("FRONTEND_URL")
            .expect("FRONTEND_URL must be set in environment variables");
        let smtp_host =
            std::env::var("SMTP_HOST").expect("SMTP_HOST must be set in environment variables");
        let smtp_port = std::env::var("SMTP_PORT")
            .expect("SMTP_PORT must be set in environment variables")
            .parse::<u16>()
            .expect("SMTP_PORT must be a valid u16 integer");
        let smtp_username = std::env::var("SMTP_USERNAME")
            .expect("SMTP_USERNAME must be set in environment variables");
        let smtp_password = std::env::var("SMTP_PASSWORD")
            .expect("SMTP_PASSWORD must be set in environment variables");
        let smtp_from_email = std::env::var("SMTP_FROM_EMAIL")
            .expect("SMTP_FROM_EMAIL must be set in environment variables");
        let smtp_from_name = std::env::var("SMTP_FROM_NAME")
            .expect("SMTP_FROM_NAME must be set in environment variables");

        Config {
            database_url,
            secret_key,
            frontend_url,
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            smtp_from_email,
            smtp_from_name,
        }
    }
}
