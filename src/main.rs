#[macro_use]
extern crate rocket;

use std::io;

use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{Duration, sleep};
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec},
    PrometheusMetrics
};
use once_cell::sync::Lazy;
use ella::cli::CLI;
use ella::ella_metrics;
use ella::metric_client::MetricClient;
use ella::metric_endpoint::MetricEndpoint;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::with_included_self_metrics();
    let endpoints: Vec<MetricEndpoint> = cli
        .metric_urls
        .into_iter()
        .filter_map(|url| MetricEndpoint::from_string(url).ok())
        .collect();

    let mut metric_client = MetricClient::with_request_client(reqwest::Client::new());
    let mut interval = tokio::time::interval(*cli.collect_interval);

    // Rocket && API
    let prometheus: PrometheusMetrics = PrometheusMetrics::new();
    prometheus.registry().
        register(Box::new(ella_metrics::ELLA_REQUEST_COUNTER.clone()))
        .unwrap();
        
    let rocket = rocket::build()
        .attach(prometheus.clone())
        .mount("/", routes![index, delay, blocking_readme])
        .mount("/metrics", prometheus);

    // Collection
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            metric_client.collect_from_endpoints(&endpoints).await;
        }
    });

    rocket.launch().await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/readme")]
async fn blocking_readme() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("README.md"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}
