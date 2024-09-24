use common::APP_HOST;
use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;
#[test]
fn test_get_all() {
    let client = Client::new();
    let user1 = common::create_user(&client);
    let user2 = common::create_user(&client);
    let response = client.get(format!("{}/users",APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&user1));
    assert!(json.as_array().unwrap().contains(&user2));

    common::delete_user(&client, user1);
    common::delete_user(&client, user2);
}

#[test]
fn test_create() {
    let client = Client::new();
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
    assert_eq!(response.status(), StatusCode::CREATED);

    let user: Value = response.json().unwrap();
    assert_eq!(user, json!({
        "id": user["id"],
        "email": "w5vXn@example.com",
        "password": "password",
        "first_name": "Test",
        "last_name": "User",
        "phone_number": "555-555-5555",
        "oauth_provider": user["oauth_provider"],
        "oauth_id": user["oauth_id"],
        "profile_picture_url": "https://example.com/image.png",
        "role": user["role"],
        "created_at": user["created_at"],
        "updated_at": user["updated_at"],
    }));

    common::delete_user(&client, user);
}

#[test]
fn test_get_one() {
    let client = Client::new();
    let user = common::create_user(&client);

    let response = client.get(format!("{}/users/{}",APP_HOST, user["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let user: Value = response.json().unwrap();
    assert_eq!(user, json!({
        "id": user["id"],
        "email": "w5vXn@example.com",
        "password": "password",
        "first_name": "Test",
        "last_name": "User",
        "phone_number": "555-555-5555",
        "oauth_provider": user["oauth_provider"],
        "oauth_id": user["oauth_id"],
        "profile_picture_url": "https://example.com/image.png",
        "role": user["role"],
        "created_at": user["created_at"],
        "updated_at": user["updated_at"],
    }));

    common::delete_user(&client, user);
}

#[test]
fn test_update() {
    let client = Client::new();
    let user = common::create_user(&client);

    let response = client.put(format!("{}/users/{}",APP_HOST, user["id"]))
        .json(&json!({
            "first_name": "Updated"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let updated_user: Value = response.json().unwrap();
    assert_eq!(updated_user["first_name"], "Updated");

    common::delete_user(&client, user);
}

#[test]
fn test_delete() {
    let client = Client::new();
    let user = common::create_user(&client);

    let response = client.delete(format!("{}/users/{}",APP_HOST, user["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}