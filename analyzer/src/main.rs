use scylla::{ SessionBuilder};
use scylla::transport::Compression;
use std::error::Error;
use tokio;
use std::time::{Duration, SystemTime};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let uri = std::env::var("SCYLLA_CONTACT_POINTS")
    .unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    
    let session = SessionBuilder::new()
        .known_node(uri)
        .compression(Some(Compression::Snappy))
        .build()
        .await?;

    let mut total_users = 0;
    let mut last_processed_time = SystemTime::now();
        
    loop {

        // Calculate the last processed timestamp
        let last_processed_str = last_processed_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64; // Convert to i64

        let query = format!(
            "SELECT id, name, created_at FROM ks.big_data_demo_table WHERE created_at > {} ALLOW FILTERING", last_processed_str);

         // Query data
        if let Some(rows) = session
        .query(query, &[])
        .await?
        .rows{
            for row in rows{

                println!("ID:");
                if let Some(id_column) = row.columns.get(0) {
                    if let Some(id) = id_column.as_ref().and_then(|col| col.as_uuid()) {

                        total_users += 1;

                        if total_users > 0 {
                            println!("Active Users {}, after adding recent user {}", total_users, id);
                        } 
                    } else {
                        println!("(NULL)");
                    }
                } else {
                    println!("Column not found");
                }

                println!("Name:");
                if let Some(name_column) = row.columns.get(1) {
                    if let Some(name) = name_column.as_ref().and_then(|col| col.as_text()) {
                        println!("{}", name);
                    } else {
                        println!("(NULL)");
                    }
                } else {
                    println!("Column not found");
                }
                
                // Update the last processed timestamp
                last_processed_time = SystemTime::now();

                // Perform your data processing logic here
            }
        };
       
        // Add a delay between iterations
        tokio::time::sleep(Duration::from_secs(10)).await; // Adjust the delay as needed
    }    

}
