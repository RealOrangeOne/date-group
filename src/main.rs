use std::path::PathBuf;
use structopt::StructOpt;
use glob::glob;
use std::fs::File;
use std::io::BufReader;
use exif::{In, Reader, Tag};
use chrono::NaiveDateTime;
use dtparse;
use std::collections::HashMap;


#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(name="source", parse(from_os_str))]
    sources: Vec<PathBuf>,
}

fn parse_datetime(date_time: String) -> Option<NaiveDateTime> {
    let parser = dtparse::Parser::default();
    return match parser.parse(&date_time, None, None, true, false, None, false, &HashMap::new()) {
        Ok((dt, _, _)) => Some(dt),
        Err(_) => None
    };
}

fn read_exif_date(file_path: &PathBuf) -> Option<NaiveDateTime> {
    let file = File::open(file_path).expect("File not found");
    let exif = Reader::new().read_from_container(&mut BufReader::new(&file)).expect("Failed to read");
    let val = exif.get_field(Tag::DateTime, In::PRIMARY).expect("Failed to get datetime");
    return parse_datetime(val.display_value().to_string());
}

fn get_filename(file_path: &PathBuf) -> Option<String> {
    let file_name = file_path.file_name()?;
    let file_name_str = file_name.to_str()?;
    return Some(String::from(file_name_str));
}

fn read_filename(file_path: &PathBuf) -> Option<NaiveDateTime> {
    let filename = get_filename(file_path)?;
    return parse_datetime(filename);
}

fn process_file(file_path: PathBuf) {
    let exif_date = read_filename(&file_path);
    println!("{:?}", exif_date);
}


fn process_directory(path: PathBuf) {
    let mut path = path.clone();
    path.push("*");
    for f in glob(path.to_str().expect("Failed to convert path")).expect("Failed to glob") {
        match f {
            Ok(f) => {
                println!("Processing {:?}", f);
                process_file(f);
            }
            Err(_) => ()
        };
    }
}


fn main() {
    let opts = Opt::from_args();
    for path in opts.sources.into_iter() {
        process_directory(path);
    }
}
