use std::path::PathBuf;
use structopt::StructOpt;
use glob::glob;
use std::fs::File;
use std::io::BufReader;
use exif::{In, Reader, Tag};
use chrono::NaiveDateTime;
use dtparse;


#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(name="source", parse(from_os_str))]
    sources: Vec<PathBuf>,
}

fn parse_datetime(date_time: String) -> Option<NaiveDateTime> {
    let parsed = dtparse::parse(&date_time);
    return match parsed {
        Ok((dt, _)) => Some(dt),
        Err(_) => None
    };
}

fn read_exif_date(file_path: &PathBuf) -> Option<NaiveDateTime> {
    let file = File::open(file_path).expect("File not found");
    let exif = Reader::new().read_from_container(&mut BufReader::new(&file)).expect("Failed to read");
    let val = exif.get_field(Tag::DateTime, In::PRIMARY).expect("Failed to get datetime");
    return parse_datetime(val.display_value().to_string());
}

fn process_file(file_path: PathBuf) {
    let exif_date = read_exif_date(&file_path);
    println!("{:?}: {:?}", file_path, exif_date);
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
