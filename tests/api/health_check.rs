use crate::helpers::{TestApp, spawn_app};
use reqwest;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let TestApp {
        address,
        db_pool: _,
        email_server: _,
    } = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    dbg!(&response);
    assert_eq!(Some(0), response.content_length());
}
