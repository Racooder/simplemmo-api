# simplemmo-api

[![Crates.io](https://img.shields.io/crates/v/simplemmo-api.svg)](https://crates.io/crates/simplemmo-api)
[![Docs.rs](https://img.shields.io/docsrs/simplemmo-api)](https://docs.rs/simplemmo-api)
[![CI](https://github.com/Racooder/simplemmo-api/actions/workflows/ci.yml/badge.svg)](https://github.com/Racooder/simplemmo-api/actions)
[![License](https://img.shields.io/crates/l/simplemmo-api.svg)](#license)
[![MSRV](https://img.shields.io/badge/rustc-1.99%2B-blue.svg)](#minimum-supported-rust-version)

An **unofficial**, async Rust client for the [SimpleMMO API](https://web.simple-mmo.com/p-api/home).

> **Disclaimer:** This crate is a community project. It is not affiliated with,
> endorsed by, or maintained by the SimpleMMO team.

## Features

- Async API built on [`reqwest`](https://crates.io/crates/reqwest) and [`tokio`](https://crates.io/crates/tokio)
- Typed request/response models with [`serde`](https://crates.io/crates/serde)
- Typed IDs (`PlayerId`, `GuildId`, `LocationId`) so you can't mix them up
- Real date/time types (`chrono::DateTime<Utc>`) instead of strings
- A single `Error` enum, including a dedicated rate-limit variant
- Selectable TLS backend (`rustls` by default, `native-tls` optional)
- Cheap to clone: share one `Client` between tasks

## Installation

```sh
cargo add simplemmo-api
```

Or add it to your `Cargo.toml`:

```toml
[dependencies]
simplemmo-api = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### Cargo features

| Feature      | Default | Description                                     |
| ------------ | ------- | ----------------------------------------------- |
| `rustls`     | yes     | Use `rustls` for TLS (no system OpenSSL needed) |
| `native-tls` | no      | Use the platform's native TLS library           |

To use the native TLS backend:

```toml
simplemmo-api = { version = "0.1", default-features = false, features = ["native-tls"] }
```

## Quick start

```rust
use simplemmo_api::Client;

#[tokio::main]
async fn main() -> simplemmo_api::Result<()> {
    let api_key = std::env::var("SIMPLEMMO_API_KEY").expect("set SIMPLEMMO_API_KEY");

    let client = Client::builder().api_key(api_key).build()?;

    let me = client.player().me().await?;
    println!("{} (level {})", me.name, me.level);
    println!("Gold: {}, Diamonds: {}", me.currencies.gold, me.currencies.diamonds);
    println!("Account created: {}", me.creation_date.format("%Y-%m-%d"));

    Ok(())
}
```

Run the bundled example:

```sh
SIMPLEMMO_API_KEY=your_key cargo run --example me
```

## API coverage

Endpoints are grouped into sub-clients: `client.player()`, `client.guild()`, and so on.

| Area       | Endpoint                   | Status         |
| ---------- | -------------------------- | -------------- |
| Player     | Your Information V2 (`me`) | ✅ Implemented |
| Player     | Player Information         | ⬜ Planned     |
| Player     | Player Equipped Items      | ⬜ Planned     |
| Player     | Player Skills              | ⬜ Planned     |
| Guild      | List of Guilds             | ⬜ Planned     |
| Guild      | Guild Information          | ⬜ Planned     |
| Guild      | Guild Members              | ⬜ Planned     |
| Guild      | Guild Member Contribution  | ⬜ Planned     |
| Guild      | Guild Wars                 | ⬜ Planned     |
| Guild      | Guild Seasons (All)        | ⬜ Planned     |
| Guild      | Guild Seasons Leaderboards | ⬜ Planned     |
| Guild      | Guild Task                 | ⬜ Planned     |
| Guild      | Guild Sanctuary            | ⬜ Planned     |
| Guild      | Guild Raid                 | ⬜ Planned     |
| Items      | Item Information           | ⬜ Planned     |
| Market     | Diamond Market             | ⬜ Planned     |
| Orphanage  | Orphanage                  | ⬜ Planned     |
| World Boss | World Bosses               | ⬜ Planned     |

## Error handling

All fallible calls return `simplemmo_api::Result<T>`. The `Error` enum
(marked `#[non_exhaustive]`) distinguishes:

| Variant                       | Meaning                                             |
| ----------------------------- | --------------------------------------------------- |
| `MissingApiKey`               | The builder was used without an API key             |
| `Url`                         | The configured base URL or a request URL is invalid |
| `Http`                        | Network or transport error                          |
| `Api { status, body }`        | The API answered with a non-success status          |
| `RateLimited { retry_after }` | The API answered with HTTP 429                      |
| `Decode { source, body }`     | The response did not match the expected model       |

`Decode` includes the raw response body, which makes it easy to report or fix
mismatches when the API changes.

```rust
use simplemmo_api::Error;

match client.player().me().await {
    Ok(me) => println!("Hello, {}!", me.name),
    Err(Error::RateLimited { retry_after }) => {
        eprintln!("Slow down, retry after {retry_after:?}");
    }
    Err(e) => eprintln!("Request failed: {e}"),
}
```

## Rate limits

According to the API documentation (as of November 17, 2026), the rate limit is 40 requests per minute.

The client does not retry automatically. When the API responds with HTTP 429,
you get `Error::RateLimited`, including the `Retry-After` value if the server
sent one.

## Date and time

All timestamps are returned as `chrono::DateTime<Utc>`. The `chrono` crate is
re-exported as `simplemmo_api::chrono`, so you can use the same version
without adding it to your own `Cargo.toml`.

## Configuration

```rust
let client = Client::builder()
    .api_key("...")
    .user_agent("my-bot/1.0")                  // optional
    .http_client(my_reqwest_client)            // optional: custom proxy, timeouts, ...
    .build()?;
```

`base_url(...)` is available for testing against a mock server.

## Minimum Supported Rust Version

The MSRV is **Rust 1.99**. Raising it is considered a minor-version change.

## Contributing

Contributions are welcome! Please open an issue first for larger changes.

```sh
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

Tests use [`wiremock`](https://crates.io/crates/wiremock); no real API key is
needed to run them. When adding an endpoint, please include a real (anonymized)
response as a fixture in `tests/fixtures/`.

[CONTRIBUTING.md](https://github.com/Racooder/simplemmo-api/blob/main/docs/CONTRIBUTING.md)

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

<ADJUST_THIS_SECTION_IF_YOU_CHOOSE_A_DIFFERENT_LICENSE>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
