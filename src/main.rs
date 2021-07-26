use color_eyre::Report;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Hello from a comfy nest we've made for ourselves");

    let client = Client::new();
    let url = "https://fasterthanli.me";

    let res = client.get(url).send().await?.error_for_status()?;
    info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
