use std::{
    io,
    path::PathBuf,
    error::Error
};

pub mod cli;
pub mod file;

use crate::file::ops::walk_dir;
use crate::cli::parser::CliParser;

// TODO:
// Make it reusable elsewhere
fn check_path(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    if path.is_absolute() {
	Ok(path)
    } else {
	let mut home = dirs::home_dir()
	    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Couldn't find HOME directory"))?;
	home.push(path);
	Ok(home)
    }
}

pub fn run(parser: CliParser) -> Result<(), Box<dyn Error>> {
    let input = check_path(parser.input.as_ref().ok_or("Couldn't parse input!")?.into())?;
    let output = check_path(parser.output.as_ref().ok_or("Couldn't parse output!")?.into())?;

    walk_dir(input, output)?;
    Ok(())
}
