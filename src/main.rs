use std::{
    env,
    error::Error,
    path::PathBuf
};

use org::file::ops::*;

fn main() -> Result<(), Box<dyn Error>> {
    let _args: Vec<String> = env::args().skip(1).collect();

    let home_dir = dirs::home_dir().expect("Could't find home directory!");
    let path1: PathBuf = home_dir.join("Pictures/test/");

    walk_dir(&path1, &path1)?;
    Ok(())
}
