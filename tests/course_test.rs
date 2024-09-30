// use common::APP_HOST;
// use reqwest::{blocking::Client, StatusCode};
// use serde_json::{json, Value};

// pub mod common;
// #[test]
// fn test_get_all() {
//     let client = Client::new();
//     let course1 = common::create_course(&client);
//     let course2 = common::create_course(&client);
//     let response = client.get(format!("{}/courses",APP_HOST)).send().unwrap();
//     assert_eq!(response.status(), StatusCode::OK);

//     let json: Value = response.json().unwrap();
//     assert!(json.as_array().unwrap().contains(&course1));
//     assert!(json.as_array().unwrap().contains(&course2));

//     common::delete_course(&client, course1);
//     common::delete_course(&client, course2);
// }

// #[test]
// fn test_create() {
//     let client = Client::new();
//     let response = client.post(format!("{}/courses",APP_HOST))
//         .json(&json!({
//             "name" : "Test Course",
//             "course_type": "ONLINE",
//             "start_date": "2022-01-01T00:00:00",
//             "end_date": "2022-01-02T00:00:00"
//         }))
//         .send()
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::CREATED);

//     let course: Value = response.json().unwrap();
//     assert_eq!(course, json!({
//         "id": course["id"],
//         "name" : "Test Course",
//         "course_type": "ONLINE",
//         "start_date": "2022-01-01T00:00:00",
//         "end_date": "2022-01-02T00:00:00",
//         "duration_in_days": 1,
//         "created_at": course["created_at"],
//         "updated_at": course["updated_at"]
//     }));

//     common::delete_course(&client, course);
// }

// #[test]
// fn test_get_one() {
//     let client = Client::new();
//     let course = common::create_course(&client);

//     let response = client.get(format!("{}/courses/{}",APP_HOST, course["id"]))
//         .send()
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::OK);

//     let course: Value = response.json().unwrap();
//     assert_eq!(course, json!({
//         "id": course["id"],
//         "name" : "Test Course",
//         "course_type": "ONLINE",
//         "start_date": "2022-01-01T00:00:00",
//         "end_date": "2022-01-02T00:00:00",
//         "duration_in_days": 1,
//         "created_at": course["created_at"],
//         "updated_at": course["updated_at"]
//     }));

//     common::delete_course(&client, course);
// }

// #[test]
// fn test_update() {
//     let client = Client::new();
//     let course = common::create_course(&client);

//     let response = client.put(format!("{}/courses/{}",APP_HOST, course["id"]))
//         .json(&json!({
//             "name": "Updated"
//         }))
//         .send()
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::OK);
//     let updated_course: Value = response.json().unwrap();
//     assert_eq!(updated_course["name"], "Updated");

//     common::delete_course(&client, course);
// }

// #[test]
// fn test_delete() {
//     let client = Client::new();
//     let course = common::create_course(&client);

//     let response = client.delete(format!("{}/courses/{}",APP_HOST, course["id"]))
//         .send()
//         .unwrap();
//     assert_eq!(response.status(), StatusCode::NO_CONTENT);
// }