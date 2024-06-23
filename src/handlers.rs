use crate::db;
use crate::models::User;
use crate::util::{get_id, get_user_request_body};
use crate::util::{INTERNAL_SERVER_ERROR, NOT_FOUND, OK_RESPONSE};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post_request(r),
                r if r.starts_with("GET /users/") => handle_get_request(r),
                r if r.starts_with("GET /users") => handle_get_all_request(),
                r if r.starts_with("PUT /users/") => handle_put_request(r),
                r if r.starts_with("DELETE /users/") => handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream
                .write_all(format!("{}{}", status_line, content).as_bytes())
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn handle_post_request(request: &str) -> (String, String) {
    match get_user_request_body(request) {
        Ok(user) => match db::insert_user(&user) {
            Ok(_) => (OK_RESPONSE.to_string(), "User created".to_string()),
            Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        },
        Err(_) => (
            INTERNAL_SERVER_ERROR.to_string(),
            "Invalid data".to_string(),
        ),
    }
}

fn handle_get_request(request: &str) -> (String, String) {
    match get_id(request).parse::<i32>() {
        Ok(id) => match db::get_user_by_id(id) {
            Ok(user) => (
                OK_RESPONSE.to_string(),
                serde_json::to_string(&user).unwrap(),
            ),
            Err(_) => (NOT_FOUND.to_string(), "User not found".to_string()),
        },
        Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Invalid ID".to_string()),
    }
}

fn handle_get_all_request() -> (String, String) {
    match db::get_all_users() {
        Ok(users) => (
            OK_RESPONSE.to_string(),
            serde_json::to_string(&users).unwrap(),
        ),
        Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

fn handle_put_request(request: &str) -> (String, String) {
    match get_id(request).parse::<i32>() {
        Ok(id) => match get_user_request_body(request) {
            Ok(user) => match db::update_user(id, &user) {
                Ok(_) => (OK_RESPONSE.to_string(), "User updated".to_string()),
                Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
            },
            Err(_) => (
                INTERNAL_SERVER_ERROR.to_string(),
                "Invalid data".to_string(),
            ),
        },
        Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Invalid ID".to_string()),
    }
}

fn handle_delete_request(request: &str) -> (String, String) {
    match get_id(request).parse::<i32>() {
        Ok(id) => match db::delete_user(id) {
            Ok(0) => (NOT_FOUND.to_string(), "User not found".to_string()),
            Ok(_) => (OK_RESPONSE.to_string(), "User deleted".to_string()),
            Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
        },
        Err(_) => (INTERNAL_SERVER_ERROR.to_string(), "Invalid ID".to_string()),
    }
}
