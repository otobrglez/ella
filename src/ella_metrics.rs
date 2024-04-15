use once_cell::sync::Lazy;
use rocket_prometheus::prometheus::{opts, IntCounterVec};

pub static ELLA_REQUEST_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(
        opts!("ella_requests", "Requests"),
        &["endpoint", "direction"],
    )
    .expect("Could not create NAME_COUNTER")
});
