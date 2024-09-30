// use common::APP_HOST;
// use reqwest::{blocking::Client, StatusCode};
// use serde_json::{json, Value};

// pub mod common;
// #[test]
// fn test_get_all() {
//     let client = Client::new();
//     let user1 = common::create_user(&client);
//     let user2 = common::create_user(&client);
//     let response = client.get(format!("{}/users",APP_HOST)).send().unwrap();
//     assert_eq!(response.status(), StatusCode::OK);

//     let json: Value = response.json().unwrap();
//     assert!(json.as_array().unwrap().contains(&user1));
//     assert!(json.as_array().unwrap().contains(&user2));

//     common::delete_user(&client, user1);
//     common::delete_user(&client, user2);
// }

// #[test]
// fn test_create() {
//     let client = Client::new();
//     let response = client.post(format!("{}/users",APP_HOST))
//         .json(&json!({
//             "first_name": "test",
//             "last_name": "create_guy",
//             "email": common::generate_random_string(),
//             "password": "password"
//         }))
//         .send()
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::CREATED);

//     let user: Value = response.json().unwrap();
//     assert_eq!(user, json!({
//         "id": user["id"],
//         "email": user["email"],
//         "password": user["password"],
//         "first_name": user["first_name"],
//         "last_name": user["last_name"],
//         "phone_number": user["phone_number"],
//         "oauth_provider": user["oauth_provider"],
//         "oauth_id": user["oauth_id"],
//         "profile_picture_url": user["profile_picture_url"],
//         "created_at": user["created_at"],
//         "updated_at": user["updated_at"],
//     }));

//     common::delete_user(&client, user);
// }

// #[test]
// fn test_get_one() {
//     let client = Client::new();
//     let user = common::create_user(&client);

//     let response = client.get(format!("{}/users/{}",APP_HOST, user["id"]))
//         .send()
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::OK);

//     let user: Value = response.json().unwrap();
//     assert_eq!(user, json!({
//         "id": user["id"],
//         "email": user["email"],
//         "password": user["password"],
//         "first_name": user["first_name"],
//         "last_name": user["last_name"],
//         "phone_number": user["phone_number"],
//         "oauth_provider": user["oauth_provider"],
//         "oauth_id": user["oauth_id"],
//         "profile_picture_url": user["profile_picture_url"],
//         "created_at": user["created_at"],
//         "updated_at": user["updated_at"],
//     }));

//     common::delete_user(&client, user);
// }

// #[test]
// fn test_update() {
//     let client = Client::new();
//     let user = common::create_user(&client);

//     let response = client.put(format!("{}/users/{}",APP_HOST, user["id"]))
//         .json(&json!({
//             "first_name": "Updated"
//         }))
//         .send()
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::OK);
//     let updated_user: Value = response.json().unwrap();
//     assert_eq!(updated_user["first_name"], "Updated");

//     common::delete_user(&client, user);
// }

// #[test]
// fn test_delete() {
//     let client = Client::new();
//     let user = common::create_user(&client);

//     let response = client.delete(format!("{}/users/{}",APP_HOST, user["id"]))
//         .send()
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::NO_CONTENT);
// }

