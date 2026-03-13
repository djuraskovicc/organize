use serde::{
    Deserialize,
    Serialize
};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileGroup {
    #[serde(rename = "FileName")]
    pub name: String, 

    #[serde(rename = "FileSize")]
    pub size: String,

    #[serde(rename = "FileModifyDate")]
    pub modify_date: Option<String>,

    #[serde(rename = "FileAccessDate")]
    pub access_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    #[serde(rename = "SourceFile")]
    pub source_file: String,
    
    #[serde(rename = "File")]
    pub file_info: Option<FileGroup>,
}

impl FileGroup {
    pub fn date_parsed(&self) -> Option<NaiveDateTime> {
        self.modify_date.as_deref().and_then(|s| {
            NaiveDateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S").ok()
        })
    }
}

impl ImageData {
    pub fn get_modify_date(&self) -> Option<&String> {
	self.file_info.as_ref()?.modify_date.as_ref()
    }
}
