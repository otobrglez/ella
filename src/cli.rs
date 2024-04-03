use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[arg(long, env, default_value = "2s")]
    pub collect_interval: humantime::Duration,

    #[arg(long, env, default_value = "1 hour")]
    retention_period: humantime::Duration,

    #[arg(long, env, default_value = "1 minute")]
    dump_period: humantime::Duration,

    #[clap(value_name = "Metric URLs", required = true)]
    pub metric_urls: Vec<String>,
}

impl CLI {
    const SELF_METRICS_URL: &'static str = "http://0.0.0.0:8000/metrics";

    fn new(
        collect_interval: humantime::Duration,
        retention_period: humantime::Duration,
        dump_period: humantime::Duration,
        metric_urls: Vec<String>,
    ) -> CLI {
        CLI {
            collect_interval,
            retention_period,
            dump_period,
            metric_urls,
        }
    }

    pub fn with_included_self_metrics() -> CLI {
        let parsed = Self::parse();
        let mut metric_urls = parsed.metric_urls;
        metric_urls.push(Self::SELF_METRICS_URL.to_string());

        Self::new(
            parsed.collect_interval,
            parsed.retention_period,
            parsed.dump_period,
            metric_urls,
        )
    }
}
