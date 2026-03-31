use std::{
    fs,
    io, 
    path::{Path, PathBuf}
};
use walkdir::WalkDir;
use nom_exif::{
    MediaParser,
    MediaSource,
    ExifIter,
    Exif,
    ExifTag,
};

fn get_time(path: &Path) -> Option<String> {
    let mut parser = MediaParser::new();
    let ms = MediaSource::file_path(path).ok()?;

    if ms.has_exif() {
        let iter: ExifIter = parser.parse(ms).ok()?;
        let exif: Exif = iter.into();

        let date_val = exif.get(ExifTag::DateTimeOriginal)
            .or_else(|| exif.get(ExifTag::ModifyDate))?;

        return date_val.as_time_components().map(|(ndt, _offset)| {
            ndt.format("%Y-%m-%d %H:%M").to_string()
        });
    }

    None
}

fn create_dirs(inpath: &PathBuf) -> io::Result<()> {
    let new_path = if inpath.is_absolute() {
	inpath.to_path_buf()
    } else {
	let mut home = dirs::home_dir()
	    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Couldn't find HOME directory"))?;
	home.push(inpath);
	home
    };

    fs::create_dir_all(&new_path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("IO Error: Couldn't create [{:?}]. Reason: {:?}", new_path, e),
        )
    })?;

    Ok(())
}

fn copy_file(path: &PathBuf, outpath: &PathBuf, time: String) -> io::Result<()> {
    // TODO:
    // Implement logic to create path based on file time creation
    // then call create_dirs function. 

    if !outpath.exists() {
	create_dirs(outpath)?;
    }

    // Function rename moves files
    fs::rename(path, outpath)?;

    Ok(())
}

fn move_file() -> io::Result<()> {
    Ok(())
}

pub fn walk_dir(inpath: &PathBuf, _outpath: &PathBuf) -> io::Result<()> {
    if !inpath.exists() {
	return Err(io::Error::new(io::ErrorKind::NotFound, "Input path doesn't exist!"));
    }

    if !inpath.is_dir() {
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input path is not a directory!"));
    }

    for entry in WalkDir::new(inpath).into_iter().filter_map(|e| e.ok()) {
	if !entry.file_type().is_file() { continue; }

	if let Some(time) = get_time(entry.path()) {
	    println!("{time}");
	} else {
	    eprintln!("Couldn't get date!");
	}
    }

    Ok(())
}
