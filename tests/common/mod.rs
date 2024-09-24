use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://localhost:8080/api";

pub fn create_user(client: &Client) -> Value {
    let response = client.post(format!("{}/users",APP_HOST))
    .json(&json!({
        "first_name": "Test",
        "last_name": "User",
        "email": "w5vXn@example.com",
        "password": "password",
        "phone_number": "555-555-5555",
        "oauth_provider": "google",
        "oauth_id": "123456789",
        "profile_picture_url": "https://example.com/image.png",
        "role": "STUDENT"
    }))
    .send()
    .unwrap();
    
    response.json().unwrap()
}

pub fn delete_user(client: &Client, user: Value) {
    let response = client.delete(format!("{}/users/{}",APP_HOST, user["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}