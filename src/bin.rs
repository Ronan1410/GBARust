#[allow(non_snake_case)]
extern crate clap;
extern crate LR35902;

use std::io::Read;
use::std::fs::File;
use clap::Parser;

use LR35902::{
    gb
};

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

    #[clap(default_value_t = 1)]
    scale: u8,
}

fn read_rom_from_path(rom_path: &String) -> Result<Vec<u8>, std::io::Error>
{
    let mut rom = Vec::new();
    let file = Fole::open(rom_path);
    match file.and_then(|mut f| f.read_to_end(&mut rom))
    {
        Ok(..) => {}
        Err(e) => {
            return Err(format!("failed to read {}: {}", filepath, e))
        }
    };
    Ok(rom)
}

fn main()
{
    let args = Args::parser();

    let rom = read_rom_from_path(&args.filepath);

    println!("{:?}", rom);

    let mut gb = gb::Gb::new();
    gb.load(rom);
}