use chrono::{DateTime, NaiveDateTime};
use dtparse::Parser;
use exif::{In, Reader, Tag};
use glob::glob;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

fn get_resolvers() -> Vec<fn(&PathBuf) -> Option<NaiveDateTime>> {
    return vec![read_exif_date, read_filename];
}

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(name = "source", parse(from_os_str))]
    sources: Vec<PathBuf>,
}

fn parse_datetime(date_time: String) -> Option<NaiveDateTime> {
    return match Parser::default().parse(
        &date_time,
        None,
        None,
        true,
        false,
        None,
        false,
        &HashMap::new(),
    ) {
        Ok((dt, _, _)) => Some(dt),
        Err(_) => None,
    };
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

fn read_filename(file_path: &PathBuf) -> Option<NaiveDateTime> {
    let file_name = file_path.file_name()?;
    let file_name_str = file_name.to_str()?;
    return parse_datetime(String::from(file_name_str));
}

fn get_date_for_file(file_path: &PathBuf) -> Option<NaiveDateTime> {
    for resolver in get_resolvers().iter() {
        let dt = resolver(file_path);
        if dt.is_some() {
            return dt;
        }
    }
    return None;
}

fn process_file(file_path: PathBuf) {
    let file_date = get_date_for_file(&file_path);
    println!("{:?}", file_date);
}

fn process_directory(path: PathBuf) {
    for f in glob(&format!("{}/*", path.display())).expect("Failed to glob") {
        if let Ok(f) = f {
            println!("Processing {:?}", f);
            process_file(f);
        }
    }
}

fn main() {
    let opts = Opt::from_args();
    for path in opts.sources.into_iter() {
        process_directory(path);
    }
}
