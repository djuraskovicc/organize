use std::{
    io,
    path::{PathBuf, Path},
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
