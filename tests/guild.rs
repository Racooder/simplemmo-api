use simplemmo_api::{Client, GuildId, models::guild::info::GuildPosition};
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
async fn list_parses_response() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/guilds/all"))
        .and(body_json(serde_json::json!({ "api_key": "test-key" })))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(include_str!("fixtures/guild/list.json"), "application/json"),
        )
        .mount(&server)
        .await;

    let list = client_for(&server).guild().list(1).await.unwrap();

    assert_eq!(list.items.len(), 20);
    assert_eq!(list.items.first().unwrap().id, GuildId(4));
}

#[tokio::test]
async fn info_parses_response() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/guilds/info/8422"))
        .and(body_json(serde_json::json!({ "api_key": "test-key" })))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(include_str!("fixtures/guild/info.json"), "application/json"),
        )
        .mount(&server)
        .await;

    let me = client_for(&server).guild().info(8422).await.unwrap();

    assert_eq!(me.name, "Test Guild");
    assert_eq!(me.experience, 552086716);
}

#[test]
fn guild_position_type_round_trips() {
    use GuildPosition::*;
    let all = [Leader, CoLeader, Officer, Member];
    for v in all {
        let s: String = v.clone().into();
        assert_eq!(GuildPosition::from(s.clone()), v, "wire string {s:?}");
        assert!(!matches!(GuildPosition::from(s), GuildPosition::Other(_)));
    }
}
