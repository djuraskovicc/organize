use std::{
    io,
    fs,
    error::Error,
    path::{PathBuf, Path},
};
use std::cmp::Ordering::{
    Greater,
    Less,
    Equal,
};
use walkdir::WalkDir;
use chrono::{
    DateTime,
    Local
};
use super::data::ImageData;

pub fn get_time(path: &Path) -> io::Result<()> {
    if !path.exists() {
	return Err(io::Error::new(io::ErrorKind::NotFound, "Path doesn't exist!"));
    }

    if !path.is_file() {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path is not a file!"));
    }

    if let Ok(metadata) = fs::metadata(path) {
	let system_time = metadata.modified()
	    .or_else(|_| metadata.created())
	    .ok();

	if let Some(time) = system_time {
	    let datetime: DateTime<Local> = time.into();
	    println!("date: {}", datetime.format("%Y-%m-%d %H:%M"));
	}
    } else {
	println!("Not supported on this platform");
    }

    Ok(())
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

pub fn walk_dir(inpath: &PathBuf, outpath: &PathBuf) -> io::Result<()> {
    if !inpath.exists() {
	return Err(io::Error::new(io::ErrorKind::NotFound, "Input path doesn't exist!"));
    }

    if !inpath.is_dir() {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input path is not a directory!"));
    }

    for entry in WalkDir::new(inpath).into_iter().filter_map(|e| e.ok()) {
	if !entry.file_type().is_file() { continue; }

	get_time(entry.path());
    }

    Ok(())
}
