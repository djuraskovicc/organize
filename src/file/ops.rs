use std::{
    error::Error,
    path::PathBuf,
};
use std::cmp::Ordering::{
    Greater,
    Less,
    Equal,
};
use exiftool::ExifTool;
use super::data::ImageData;

pub fn parse_serialize(path: &PathBuf) -> Result<ImageData, Box<dyn Error>> {
    let exiftool = ExifTool::new()?;
    let exif_data = exiftool.json(path, &["-g"])?;

    let obj: ImageData = serde_json::from_value(exif_data)?;
    Ok(obj)
}

pub fn compare_timedates(file1: &ImageData, file2: &ImageData) {
    let (Some(a), Some(b)) = (file1.get_modify_date(), file2.get_modify_date()) else {
	eprintln!("Missing date info!");
	return;
    };

    match a.cmp(b) {
	Greater => println!("File A is newer"),
	Less    => println!("File B is newer"),
	Equal   => println!("Identical"),
    }
}
