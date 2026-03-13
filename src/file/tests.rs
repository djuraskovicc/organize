#[allow(unused)]
use std::{
    path::PathBuf,
    error::Error,
};
#[allow(unused)]
use super::{
    ops::*,
    data::*,
};

#[test]
fn test_metadata() -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().expect("Could't find home directory!");
    let path: PathBuf = home_dir.join("Pictures/linux.jpg");

    let data = parse_metadata(&path)?;
    assert!(!data.is_empty());
    Ok(())
}

#[test]
fn test_serialize() -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().expect("Could't find home directory!");
    let path: PathBuf = home_dir.join("Pictures/linux.jpg");

    compare_timedate(&path)?;
    Ok(())
}
