#![allow(non_snake_case)]

use std::fs;
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

fn read_rom_from_path(rom_path: &String) -> Result<Vec<u8>, std::io::Error>
{
    match fs::read(rom_path)
    {
        Ok(rom) => Ok(rom),
        Err(_) => Err(format!("Could not open file {}", rom_path)),
    }
}

fn main()
{
    let args = Args::parser();

    let rom = read_rom_from_path(&args.filepath);

    println!("{:?}", rom);
}