use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}};
use anyhow::Result;

pub type Database = Surreal<Client>;

pub async fn initialize_database(
    endpoint: &str,
    namespace: &str,
    database: &str,
) -> Result<Database> {
    let db = Surreal::new::<Ws>(endpoint).await?;
    
    // Sign in as root user (for development)
    db.signin(surrealdb::opt::auth::Root {
        username: "root",
        password: "root",
    }).await?;
    
    // Use namespace and database
    db.use_ns(namespace).use_db(database).await?;
    
    Ok(db)
}