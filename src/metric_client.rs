use std::time::Duration;

use futures::future::join_all;
use log::error;
use reqwest::Client;
use rocket::futures::stream;

use crate::{ella_metrics, prom_to_json};
use crate::metric_endpoint::MetricEndpoint;
use crate::metric_parser::MetricParser;

pub struct MetricClient {
    client: Client,
}

impl MetricClient {
    pub fn with_request_client(client: Client) -> MetricClient {
        Self { client }
    }

    pub async fn collect_from_endpoints(&mut self, endpoints: &Vec<MetricEndpoint>) -> () {
        let fetch_tasks = endpoints.iter().map(|endpoint| {
            let client = &self.client;
            async move { MetricClient::collect_endpoint(client, endpoint).await }
        });

        let _results = join_all(fetch_tasks).await;
        ()
    }

    async fn collect_endpoint(client: &Client, metric_endpoint: &MetricEndpoint) {
        ella_metrics::ELLA_REQUEST_COUNTER
            .with_label_values(&[metric_endpoint.url.as_str(), "sent"]).inc();

        let result = client
            .get(metric_endpoint.url.as_str())
            .timeout(Duration::from_secs(3))
            .send()
            .await;
        match result {
            Ok(response) => {
                ella_metrics::ELLA_REQUEST_COUNTER
                    .with_label_values(&[metric_endpoint.url.as_str(), "received-ok"]).inc();

                // TODO: This needs way more work.
                let body = response.text().await.unwrap();
                // let result = prom_to_json::parse(body).unwrap();

                let _result = Self::parse_metrics(body);

            }
            Err(error) => {
                ella_metrics::ELLA_REQUEST_COUNTER
                    .with_label_values(&[metric_endpoint.url.as_str(), "received-error"]).inc();
                error!("[{}] - Failed: {}", metric_endpoint.url, error)
            },
        }

        ()
    }

    fn parse_metrics(raw: String) -> () {
        let result = MetricParser::prom_to_families(raw).unwrap();
        // let json = serde_json::to_string_pretty(&result).unwrap();
        // let json = serde_json::to_string(&result).unwrap();
        println!("Families size: {}", result.len());
    }
}
