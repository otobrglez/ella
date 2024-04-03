use clap::Parser;

#[derive(Parser, Debug)]
pub struct MetricEndpoint {
    pub url: String,
}

impl MetricEndpoint {
    pub fn from_string(raw: String) -> Result<MetricEndpoint, Box<dyn std::error::Error>> {
        if raw.is_empty() {
            Err(Box::from("Url is empty"))
        } else {
            Ok(MetricEndpoint { url: raw })
        }
    }
}
