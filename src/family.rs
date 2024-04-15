use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

use crate::utils::f64_from_string;

pub type Labels = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueMetric {
    #[serde(deserialize_with = "f64_from_string")]
    value: f64,
    #[serde(default = "empty_labels")]
    labels: Labels,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistogramMetric {
    #[serde(deserialize_with = "f64_from_string")]
    count: f64,
    #[serde(deserialize_with = "f64_from_string")]
    sum: f64,

    #[serde(default = "empty_labels")]
    labels: Labels,
    /*
    TODO: This is missing.
    #[serde(deserialize_with = "f64_from_string")]
    buckets: HashMap<String, f64>,
     */
}

fn empty_labels() -> Labels {
    HashMap::new()
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Family {
    #[serde(rename(deserialize = "COUNTER"))]
    Counter {
        name: String,
        help: Option<String>,
        metrics: Vec<ValueMetric>,
    },

    #[serde(rename(deserialize = "HISTOGRAM"))]
    Histogram {
        name: String,
        help: Option<String>,
        metrics: Vec<HistogramMetric>,
    },

    #[serde(rename(deserialize = "GAUGE"))]
    Gauge { name: String, help: Option<String> },

    #[serde(rename(deserialize = "SUMMARY"))]
    Summary { name: String, help: Option<String> },

    #[serde(rename(deserialize = "UNTYPED"))]
    Untyped { name: String, help: Option<String> },
}
