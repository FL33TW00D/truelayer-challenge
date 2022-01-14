use crate::helpers::TestApp;
#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let app = TestApp::new().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    //28 == "Houston, we have a lift off!" ;)
    assert_eq!(Some(28), response.content_length());
}
