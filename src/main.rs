use color_eyre::Report;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
pub const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Building that fetch future...");
    let client = Client::new();
    let fut = fetch_thing(&client, URL_1);
    info!("Sleeping for a bit...");
    sleep(Duration::from_secs(1)).await;
    info!("Awaiting that fetch future...");
    fut.await?;
    info!("Done awaiting that fetch future");

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
        //.json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn type_name_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

async fn fetch_thing(client: &Client, url: &str) -> Result<(), Report> {
    let res = client.get(url).send().await?.error_for_status()?;
    info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
    Ok(())
}
