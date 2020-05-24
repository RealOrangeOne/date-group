use std::path::PathBuf;
use structopt::StructOpt;
use glob::glob;
use std::fs::File;
use std::io::BufReader;
use exif::{In, Reader, Tag};


#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(name="source", parse(from_os_str))]
    sources: Vec<PathBuf>,
}

fn process_file(file_path: PathBuf) -> String {
    println!("{:?}", file_path);
    let file = File::open(&file_path).expect("File not found");
    let exif = Reader::new().read_from_container(&mut BufReader::new(&file)).expect("Failed to read");
    let val = exif.get_field(Tag::DateTime, In::PRIMARY).expect("Failed to get datetime");
    return val.display_value().to_string();
}

fn process_directory(path: PathBuf) {
    let mut path = path.clone();
    path.push("*");
    for f in glob(path.to_str().expect("Failed to convert path")).expect("Failed to glob") {
        match f {
            Ok(f) => process_file(f),
            Err(_) => String::new()
        };
    }
}


fn main() {
    let opts = Opt::from_args();
    for path in opts.sources.into_iter() {
        process_directory(path);
    }
}
