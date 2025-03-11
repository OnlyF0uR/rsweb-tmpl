use std::sync::Arc;

use rsweb_database::DB;
use serde::Deserialize;
use sqlx::types::time::Date;
use time_macros::format_description;
use tokio::fs;

#[derive(Debug, Deserialize)]
struct JsonData {
    #[serde(default)]
    users: Vec<JsonUser>,
}

#[derive(Debug, Deserialize)]
struct JsonUser {
    email: Option<String>,
    google_sub: Option<String>,
    handle: String,
    gender: String,
    dob: String,
    role: String,
}

pub async fn insert_data(db: &Arc<DB>) {
    // Read the json file
    let json_content = fs::read_to_string("simulate.json")
        .await
        .expect("Failed to read JSON file");

    let json_data: JsonData =
        serde_json::from_str(&json_content).expect("Failed to parse JSON file");

    // Insert users
    insert_users(db, json_data.users.as_slice()).await;
}

async fn insert_users(db: &Arc<DB>, users: &[JsonUser]) {
    for user in users {
        let dob = Date::parse(&user.dob, format_description!("[year]-[month]-[day]"))
            .expect("Failed to parse date");

        sqlx::query!("INSERT INTO users (email, handle, gender, date_of_birth, role, google_sub) VALUES ($1, $2, $3, $4, $5, $6)",
            user.email,
            user.handle,
            user.gender,
            dob,
            user.role,
            user.google_sub
        ).execute(&db.pool).await.expect("Failed to insert user");
    }

    println!("Inserted {} users", users.len());
}
