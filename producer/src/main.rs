use scylla::{ SessionBuilder};
use scylla::transport::Compression;
use rand::Rng;
use uuid::Uuid;
use std::time::Duration;
use tokio::time::sleep;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let uri = std::env::var("SCYLLA_CONTACT_POINTS")
    .unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    
    let session = SessionBuilder::new()
        .known_node(uri)
        .compression(Some(Compression::Snappy))
        .build()
        .await?;

    // Create the keyspace if It doesn't exist
    session
    .query(
        "CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = \
        {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
        &[],
    )
    .await?;
        
    // Use the keyspace
    session
        .query("USE ks", &[],)
        .await?;

    // toTimestamp(now())
    // Create a Table if doesn't exist
    session
        .query("CREATE TABLE IF NOT EXISTS ks.big_data_demo_table (ID UUID PRIMARY KEY, NAME TEXT , created_at TIMESTAMP)", &[],)
        .await?;

    loop {
        let id = Uuid::new_v4();
        let name = format!("User{}", id);

        let name_clone = name.clone();

        session
            .query(
                "INSERT INTO ks.big_data_demo_table (id, name, created_at) VALUES (?, ?, toTimestamp(now()))",
                (id, name_clone),
            )
            .await?;

        println!("Inserted: ID {}, Name {}", id, name);

        let delay = rand::thread_rng().gen_range(1000..5000); // Simulate data generation time
        sleep(Duration::from_millis(delay)).await;
    }

}
