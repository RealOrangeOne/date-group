use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(name="source", parse(from_os_str))]
    sources: Vec<PathBuf>,
}


fn main() {
    let opts = Opt::from_args();
    for elem in opts.sources.into_iter() {
        println!("{:?}", elem)
    }
}
