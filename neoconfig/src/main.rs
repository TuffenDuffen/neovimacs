use crate::cli::Settings;
use clap::Parser;

mod cli;
mod lua_api;
mod lua_reader;

fn main() {
    println!("Hello, world!");
    let settings = Settings::parse();
    let system_config = lua_reader::get_config(&settings);
}
