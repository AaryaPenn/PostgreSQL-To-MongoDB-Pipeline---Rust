use tokio::stream::StreamExt;
use tokio_postgres::{Config, NoTls};
use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // PostgreSQL connection parameters
    let pg_config = "host=localhost user=username password=password dbname=mydatabase";
    let pg_conn = Config::from_str(pg_config)?.connect(NoTls).await?;

    // MongoDB connection parameters
    let mongo_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(mongo_uri)?;
    let mongo_client = Client::with_options(client_options)?;

    // Specify the PostgreSQL table to migrate
    let pg_table_name = "your_postgres_table";
    
    // Fetch data from PostgreSQL
    let pg_rows = pg_conn
        .query(&format!("SELECT * FROM {}", pg_table_name), &[])
        .await?;

    // MongoDB collection to insert data
    let mongo_db = mongo_client.database("your_mongo_db");
    let mongo_collection = mongo_db.collection::<Document>("your_mongo_collection");

    // Iterate over PostgreSQL rows and insert into MongoDB
    for row in pg_rows {
        let document = create_mongo_document(&row)?;
        mongo_collection.insert_one(document, None).await?;
    }

    println!("Migration completed successfully");
    
    Ok(())
}

fn create_mongo_document(pg_row: &tokio_postgres::Row) -> Result<Document, Box<dyn Error>> {
    // Customize this function based on your data transformation needs
    let field1_value: String = pg_row.try_get("field1")?;
    let field2_value: i32 = pg_row.try_get("field2")?;

    // Create a MongoDB document from PostgreSQL data
    let mongo_document = doc! {
        "field1": field1_value,
        "field2": field2_value,
        // Add more fields as needed
    };

    Ok(mongo_document)
}
