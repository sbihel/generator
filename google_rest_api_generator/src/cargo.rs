const CARGO_TOML: &str = r#"
[package]
name = "{crate_name}"
version = "{crate_version}"
authors = ["Glenn Griffin <ggriffiniii@gmail.com"]
edition = "2018"
# for now, let's not even accidentally publish these
publish = false

[features]
default = ["rustls-tls"]

native-tls = ["reqwest/native-tls"]
rustls-tls = ["reqwest/rustls-tls"]

[dependencies]
chrono = { version = "0.4", features = ["serde"] }
futures = "0.3"
google_api_auth = { git = "https://github.com/google-apis-rs/generator", branch = "refactor/async" }
google_field_selector = { git = "https://github.com/google-apis-rs/generator", branch = "refactor/async" }
mime = "0.3"
percent-encoding = "2"
reqwest = { version = "0.10", default-features = false, features = ["json", "stream"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
textnonce = "0.6"
"#;

pub(crate) fn cargo_toml(crate_name: &str, include_bytes_dep: bool, api: &shared::Api) -> String {
    let mut doc = CARGO_TOML
        .trim()
        .replace("{crate_name}", crate_name)
        .replace(
            "{crate_version}",
            &api.lib_crate_version
                .as_ref()
                .expect("available crate version"),
        );

    // TODO: figure out a better way to determine if we should include stream reqwest feature
    if crate_name.contains("storage") {
        doc = doc.replace(r#"features = ["json"]"#, r#"features = ["stream", "json"]"#);
    }

    if include_bytes_dep {
        doc.push_str("\n\n[dependencies.google_api_bytes]\n");
        doc.push_str("git = \"https://github.com/google-apis-rs/generator\"\n");
    }

    doc
}
