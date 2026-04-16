#![allow(non_snake_case)]

use clap::Parser;

#[derive(Parser, Debug)]

struct Args
{
    #[clap(default_value_t = String::from(""))]
    filepath: String,

    #[clap(default_value_t = true, short, long)]
    browser: bool,

    #[clap(short, long)]
    desktop: bool,

    #[clap(short, long)]
    audio: bool,
}

fn main()
{
    let args = Args::parser();

    println!("{:?}", args);
}