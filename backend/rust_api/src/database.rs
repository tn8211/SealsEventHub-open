use rocket::tokio::{self, sync::Mutex};
use tokio_postgres::{NoTls, Client, Error};
use std::env;
pub struct DbClient {
    pub client: Mutex<Client>,
}

impl DbClient {
    pub async fn connect() -> Result<DbClient, Error> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://root:groot@localhost:5000/postgres_db".to_string());
        
        let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

        // Spawn the connection on a separate task to handle it asynchronously
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(DbClient {
            client: Mutex::new(client),
        })
    }
}
