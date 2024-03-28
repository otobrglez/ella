#[macro_use] extern crate rocket;
use clap::Parser;
use std::ffi::{CStr, CString};
use ella::*;
use rocket::tokio::time::{sleep, Duration};
use std::io;
use rocket::tokio::task::spawn_blocking;
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec},
    PrometheusMetrics,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[arg(short, long, env, default_value = "3434")]
    port: usize,

    #[clap(value_name = "Metric URLs", required = true)]
    metric_urls: Vec<String>,
}

fn old_main() {

    /*
    println!("This is ella");
    let args = &CLI::parse();
    dbg!("Collecting {:#?}", args);
    println!("{:#?}", args.port);

    println!("Do some add here");
    */

    unsafe {
        let x: GoInt = 10;
        let y: GoInt = 20;
        let r: GoInt = add_numbers(x, y);
        println!("Hello {:?}", r)
    }
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
    let vec = spawn_blocking(|| std::fs::read("README.md")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

/*
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
*/

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let prometheus: PrometheusMetrics = PrometheusMetrics::new();

    let _rocket = rocket::build()
        .attach(prometheus.clone())
        .mount("/", routes![index, delay, blocking_readme])
        .mount("/metrics", prometheus)
        .launch()
        .await?;

    Ok(())
}
