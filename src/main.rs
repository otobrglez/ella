use clap::Parser;
use std::ffi::{CStr, CString};
use ella::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[arg(short, long, env, default_value = "3434")]
    port: usize,

    #[clap(value_name = "Metric URLs", required = true)]
    metric_urls: Vec<String>,
}

fn main() {

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
