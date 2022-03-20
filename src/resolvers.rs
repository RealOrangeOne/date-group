use crate::utils::parse_datetime;
use chrono::{DateTime, NaiveDateTime};
use exif::{In, Reader, Tag};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

lazy_static! {
    static ref RESOLVERS: Vec<fn(&Path) -> Option<NaiveDateTime>> =
        vec![read_exif_date, read_filename];
}

fn read_filename(file_path: &Path) -> Option<NaiveDateTime> {
    let file_name = file_path.file_name()?;
    let file_name_str = file_name.to_str()?;
    return parse_datetime(String::from(file_name_str));
}

fn read_exif_date(file_path: &Path) -> Option<NaiveDateTime> {
    let file = File::open(file_path).expect("File not found");
    let exif = Reader::new()
        .read_from_container(&mut BufReader::new(&file))
        .ok()?;
    let val = exif.get_field(Tag::DateTime, In::PRIMARY)?;
    return DateTime::parse_from_rfc2822(&val.display_value().to_string())
        .map(|dt| dt.naive_local())
        .ok();
}

pub fn get_date_for_file(file_path: &Path) -> Option<NaiveDateTime> {
    for resolver in RESOLVERS.iter() {
        let dt = resolver(file_path);
        if dt.is_some() {
            return dt;
        }
    }
    return None;
}
