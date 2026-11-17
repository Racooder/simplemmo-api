# Contributing to simplemmo-api

Thanks for your interest in improving `simplemmo-api`! This is a small
community project, and contributions of any size are welcome: bug reports,
documentation fixes, new endpoints, and tests.

## Before you start

- **Bugs and ideas:** Please open an issue first for anything bigger than a
  small fix, so we can agree on the approach before you spend time on it.
- **Security issues:** Don't open a public issue. Contact racooder@racoonia.net instead.
- **Be kind**

<!--TODO: Code of Conduct link after Be kind:-->

## Development setup

You need a stable Rust toolchain (see the MSRV in `Cargo.toml` under
`rust-version`). No API key is required to run the tests.

```sh
git clone https://github.com/Racooder/simplemmo-api
cd simplemmo-api
cargo test
```

To try the code against the real API:

```sh
SIMPLEMMO_API_KEY=your_key cargo run --example me
```

Never commit API keys or real responses containing private data.

## Checks that must pass

CI runs the same commands. Please run them locally before opening a pull request:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Adding a new endpoint

The project follows one pattern for every endpoint:

1. **Model:** Add the response types to `src/models/<area>.rs`.
   Derive `Debug, Clone, Serialize, Deserialize`, mark public structs
   `#[non_exhaustive]`, and use `Option<T>` for fields that can be `null` or missing.
2. **API method:** Add a method to the matching sub-client in `src/api/<area>.rs`
   (create the file and a `Client::<area>()` accessor if the area is new).
   All HTTP work goes through `Client::post`; don't build requests elsewhere.
3. **IDs and times:** Use the typed IDs (`PlayerId`, `GuildId`, ...) and
   `chrono::DateTime<Utc>` for timestamps.
4. **Test:** Add a `wiremock` test in `tests/`, using a real, anonymized response
   saved as `tests/fixtures/<name>.json`.
5. **Docs:** Add a doc comment with the official endpoint name, and update the
   coverage table in the README.

## Pull requests

- Keep each PR focused on one change.
- Describe what changed and why; link the related issue if there is one.
- Add tests for new behavior and bug fixes.
- Update `CHANGELOG.md` under "Unreleased".
- Don't raise the MSRV in a PR unless it is the point of the PR.

## AI-assisted contributions

AI tools are allowed, but you are responsible for every line you submit. Please understand and test your code, and mention significant AI use in the PR description.

## License

By contributing, you agree that your contributions will be licensed under the
same terms as the project: MIT OR Apache-2.0 (see the README).
