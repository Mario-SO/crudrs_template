use crate::models::User;
use postgres::{Client, Error as PostgresError, NoTls};
use std::env;

pub fn set_database() -> Result<(), PostgresError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&db_url, NoTls)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )",
    )?;
    Ok(())
}

pub fn insert_user(user: &User) -> Result<(), PostgresError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&db_url, NoTls)?;
    client.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email],
    )?;
    Ok(())
}

pub fn get_user_by_id(user_id: i32) -> Result<User, PostgresError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&db_url, NoTls)?;
    let row = client.query_one("SELECT * FROM users WHERE id = $1", &[&user_id])?;
    Ok(User {
        id: Some(row.get(0)),
        name: row.get(1),
        email: row.get(2),
    })
}

pub fn get_all_users() -> Result<Vec<User>, PostgresError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&db_url, NoTls)?;
    let mut users = Vec::new();
    for row in client.query("SELECT * FROM users", &[])? {
        users.push(User {
            id: Some(row.get(0)),
            name: row.get(1),
            email: row.get(2),
        });
    }
    Ok(users)
}

pub fn update_user(user_id: i32, user: &User) -> Result<(), PostgresError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&db_url, NoTls)?;
    client.execute(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        &[&user.name, &user.email, &user_id],
    )?;
    Ok(())
}

pub fn delete_user(user_id: i32) -> Result<usize, PostgresError> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client = Client::connect(&db_url, NoTls)?;
    let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&user_id])?;
    Ok(rows_affected as usize)
}
