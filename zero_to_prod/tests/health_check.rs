use std::time::Duration;
use tide_testing::TideTestingExt;

#[async_std::test]
async fn health_check_success() {
    // set up app

    let server = zero_to_prod::setup_app();

    let resp = server
        .get("/health_check")
        .await;

    assert!(resp.is_ok());
    let resp = resp.unwrap();

    assert!(resp.status().is_success());
    assert_eq!(resp.len(), Some(0));
}