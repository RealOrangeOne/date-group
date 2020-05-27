use crate::utils::parse_datetime;
use chrono::{DateTime, NaiveDateTime};
use exif::{In, Reader, Tag};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[inline]
fn get_resolvers() -> Vec<fn(&PathBuf) -> Option<NaiveDateTime>> {
    return vec![read_exif_date, read_filename];
}

fn read_filename(file_path: &PathBuf) -> Option<NaiveDateTime> {
    let file_name = file_path.file_name()?;
    let file_name_str = file_name.to_str()?;
    return parse_datetime(String::from(file_name_str));
}

fn read_exif_date(file_path: &PathBuf) -> Option<NaiveDateTime> {
    let file = File::open(file_path).expect("File not found");
    let exif = Reader::new()
        .read_from_container(&mut BufReader::new(&file))
        .ok()?;
    let val = exif.get_field(Tag::DateTime, In::PRIMARY)?;
    return DateTime::parse_from_rfc2822(&val.display_value().to_string())
        .map(|dt| dt.naive_local())
        .ok();
}

pub fn get_date_for_file(file_path: &PathBuf) -> Option<NaiveDateTime> {
    for resolver in get_resolvers().iter() {
        let dt = resolver(file_path);
        if dt.is_some() {
            return dt;
        }
    }
    return None;
}
