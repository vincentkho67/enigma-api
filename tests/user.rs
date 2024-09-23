use reqwest::{blocking::Client, StatusCode};

#[test]
fn test_get_all() {
    let client = Client::new();
    let response = client.get("http://localhost:8080/api/users").send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}