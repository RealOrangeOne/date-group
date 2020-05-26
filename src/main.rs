use chrono::{DateTime, NaiveDateTime};
use dtparse::Parser;
use exif::{In, Reader, Tag};
use glob::glob;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{create_dir_all, rename, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

fn sleep(time: u64) {
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_millis(time));
}

fn get_resolvers() -> Vec<fn(&PathBuf) -> Option<NaiveDateTime>> {
    return vec![read_exif_date, read_filename];
}

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long)]
    dry_run: bool,

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

fn process_file(file_path: &PathBuf, root: &Path, dry_run: bool) -> Option<PathBuf> {
    let file_date = get_date_for_file(file_path);
    if let Some(date) = file_date {
        let out_path = root
            .join(date.format("%Y/%B").to_string())
            .join(file_path.file_name()?);
        if out_path.exists() {
            return None;
        }
        if !dry_run {
            if let Some(parent) = out_path.parent() {
                create_dir_all(parent).expect("Failed to create directory");
            }
            rename(&file_path, &out_path).unwrap();
        }
        return Some(out_path);
    }
    return None;
}

fn list_directories(directories: Vec<PathBuf>) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut directory_map = HashMap::with_capacity(directories.len());
    for directory in directories.into_iter() {
        directory_map.insert(
            directory.clone(),
            glob(&format!("{}/**/*.*", directory.display()))
                .map_or(Vec::with_capacity(0), |paths| {
                    paths.filter_map(|r| r.ok()).collect()
                }),
        );
    }
    return directory_map;
}

fn main() {
    let opts = Opt::from_args();
    let directory_map = list_directories(opts.sources);
    let file_count = directory_map.values().flatten().count();
    let pb = ProgressBar::new(file_count.try_into().expect("Too many files"));

    for (directory, files) in directory_map.iter() {
        for file in files.into_iter() {
            sleep(100);
            let out_path = process_file(file, directory, opts.dry_run);
            match out_path {
                Some(out) => pb.println(format!("{} -> {}", file.display(), out.display())),
                None => pb.println(format!("Failed to parse date for {}", file.display())),
            }
            pb.inc(1);
        }
    }
    pb.finish();
}
