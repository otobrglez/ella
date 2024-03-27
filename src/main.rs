use clap::Parser;
// use prom2json::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[arg(short, long, env, default_value = "3434")]
    port: usize,

    #[clap(value_name = "Metric URLs", required = true)]
    metric_urls: Vec<String>,
}

fn main() {
    println!("This is ella");
    let args = &CLI::parse();
    dbg!("Collecting {:#?}", args);
    println!("{:#?}", args.port);

    println!("Do some add here");
}
