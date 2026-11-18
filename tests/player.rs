use simplemmo_api::{Client, Error};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path},
};

fn client_for(server: &MockServer) -> Client {
    Client::builder()
        .api_key("test-key")
        .base_url(format!("{}/", server.uri()))
        .build()
        .unwrap()
}

#[tokio::test]
async fn me_parses_response() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/player/me"))
        .and(body_json(serde_json::json!({ "api_key": "test-key" })))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(include_str!("fixtures/player/me.json"), "application/json"),
        )
        .mount(&server)
        .await;

    let me = client_for(&server).player().me().await.unwrap();

    assert_eq!(me.name, "Tester");
    assert_eq!(me.level, 1691);
}

#[tokio::test]
async fn info_parses_response() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/player/info/883369"))
        .and(body_json(serde_json::json!({ "api_key": "test-key" })))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("fixtures/player/info.json"),
            "application/json",
        ))
        .mount(&server)
        .await;

    let me = client_for(&server).player().info(883369).await.unwrap();

    assert_eq!(me.name, "Tester");
    assert_eq!(me.level, 1692);
}

#[tokio::test]
async fn equipment_parses_response() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/player/equipment/883369"))
        .and(body_json(serde_json::json!({ "api_key": "test-key" })))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("fixtures/player/equipment.json"),
            "application/json",
        ))
        .mount(&server)
        .await;

    let equipment = client_for(&server)
        .player()
        .equipment(883369)
        .await
        .unwrap();

    assert_eq!(equipment.len(), 13);
    assert_eq!(equipment.get(0).unwrap().name, "The Seer");
}

#[tokio::test]
async fn me_maps_rate_limit() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "7"))
        .mount(&server)
        .await;

    let err = client_for(&server).player().me().await.unwrap_err();

    assert!(matches!(err, Error::RateLimited { retry_after: Some(d) } if d.as_secs() == 7));
}
