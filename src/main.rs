use std::{
    env,
    error::Error,
    path::PathBuf
};

use org::file::ops::*;
use org::file::data::*;

fn main() -> Result<(), Box<dyn Error>> {
    let _args: Vec<String> = env::args().skip(1).collect();

    let home_dir = dirs::home_dir().expect("Could't find home directory!");
    let path1: PathBuf = home_dir.join("Pictures/linux.jpg");
    let path2: PathBuf = home_dir.join("Pictures/mili.jpg");

    let file1: ImageData = parse_serialize(&path1)?;
    let file2: ImageData = parse_serialize(&path2)?;

    compare_timedates(&file1, &file2);
    Ok(())
}
