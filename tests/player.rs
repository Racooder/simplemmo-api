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
async fn me_maps_rate_limit() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "7"))
        .mount(&server)
        .await;

    let err = client_for(&server).player().me().await.unwrap_err();

    assert!(matches!(err, Error::RateLimited { retry_after: Some(d) } if d.as_secs() == 7));
}
