#[allow(unused)]
use std::{
    path::PathBuf,
    error::Error,
};
#[allow(unused)]
use super::ops::*;

#[test]
fn test_get_dates() -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().expect("Could't find home directory!");
    let path1: PathBuf = home_dir.join("Pictures/WallPaper");

    walk_dir(&path1, &path1)?;
    Ok(())
}
