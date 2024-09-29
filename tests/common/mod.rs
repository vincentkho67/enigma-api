use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
use rand::Rng;

pub static APP_HOST: &'static str = "http://localhost:8080/api";
// Helper
pub fn generate_random_string() -> String {
    let alphabet: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(10);

    for _ in 0..10 {
        let idx = rng.gen_range(0..alphabet.len());
        result.push(alphabet[idx] as char);
    }

    result
}

// User Section
pub fn create_user(client: &Client) -> Value {
    let response = client.post(format!("{}/users",APP_HOST))
    .json(&json!({
        "first_name": "test",
        "last_name": "helloew",
        "email": generate_random_string(),
        "password": "password"
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

// Course Section
pub fn create_course(client: &Client) -> Value {
    let response = client.post(format!("{}/courses",APP_HOST))
    .json(&json!({
        "name" : "Test Course",
        "course_type": "ONLINE",
        "start_date": "2022-01-01T00:00:00",
        "end_date": "2022-01-02T00:00:00"
    }))
    .send()
    .unwrap();
    
    response.json().unwrap()
}

pub fn delete_course(client: &Client, course: Value) {
    let response = client.delete(format!("{}/courses/{}",APP_HOST, course["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}