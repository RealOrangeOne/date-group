use std::path::PathBuf;
use structopt::StructOpt;
use glob::glob;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(name="source", parse(from_os_str))]
    sources: Vec<PathBuf>,
}

fn process_file(file: PathBuf) {
    println!("{:?}", file);
}

fn process_directory(path: PathBuf) {
    let mut path = path.clone();
    path.push("*");
    for f in glob(path.to_str().expect("Failed to convert path")).expect("Failed to glob") {
        match f {
            Ok(f) => process_file(f),
            Err(_) => ()
        }
    }
}


fn main() {
    let opts = Opt::from_args();
    for path in opts.sources.into_iter() {
        process_directory(path);
    }
}
