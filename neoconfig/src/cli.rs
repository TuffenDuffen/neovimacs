use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Settings {
    /// Optional path to the lua module
    /// Default is ./neoconfig.lua
    path: Option<PathBuf>,
}
