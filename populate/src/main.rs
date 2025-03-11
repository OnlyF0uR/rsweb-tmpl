use std::sync::Arc;

use clap::Parser;
use dotenvy::dotenv;
use rsweb_database::{get_db, DB};
use simulate::insert_data;
use tokio::fs;

mod simulate;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long)]
    sim: bool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    reset_db().await;

    let db = get_db().await;
    setup_db(&db).await;

    if cli.sim {
        insert_data(&db).await;
        println!("Running in simulation mode");
    } else {
        println!("Running in production mode");
    }
}

async fn reset_db() {
    let mut database_url = std::env::var("DATABASE_URL").unwrap();
    let db_name = database_url.split("/").last().unwrap().to_string();

    // Remove the database name and connect to postgres database instead
    database_url = database_url.replace(&db_name, "postgres");
    let db = DB::from_url(database_url.as_str())
        .await
        .expect("Failed to connect to database");

    // For the activity query, we need to simplify it to a single statement
    let terminate_query = format!(
        "SELECT pg_terminate_backend(pg_stat_activity.pid)
         FROM pg_stat_activity
         WHERE pg_stat_activity.datname = '{}'",
        db_name
    );

    // Execute each query separately
    sqlx::query(&terminate_query)
        .execute(&db.pool)
        .await
        .expect("Failed to terminate connections");

    // Simple drop query
    let drop_query = format!("DROP DATABASE IF EXISTS {}", db_name);
    sqlx::query(&drop_query)
        .execute(&db.pool)
        .await
        .expect("Failed to drop database");

    println!("Database dropped");

    // Simple create query
    let create_query = format!("CREATE DATABASE {}", db_name);
    sqlx::query(&create_query)
        .execute(&db.pool)
        .await
        .expect("Failed to create database");

    println!("Database created");
}

async fn setup_db(db: &Arc<DB>) {
    // Loop through all the sql files in the sql folder and add their names to a vector
    let mut sql_files = vec![];
    for entry in std::fs::read_dir("sql").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            sql_files.push(path);
        }
    }

    // Sort the vector
    sql_files.sort();

    // Loop through the vector and execute the sql files
    for file in sql_files {
        let sql_content = fs::read_to_string(&file)
            .await
            .expect("Failed to read SQL file");
        println!("Executing SQL file: {}", file.display());

        // Execute using raw exuctor
        sqlx::raw_sql(&sql_content)
            .execute(&db.pool)
            .await
            .expect("Failed to execute SQL file");
    }

    println!("Database setup complete");
}
