use glob::glob;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{create_dir_all, rename};
use std::path::{Path, PathBuf};
use std::thread;
use structopt::StructOpt;

mod resolvers;
mod utils;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long)]
    dry_run: bool,

    #[structopt(long, default_value = "%Y/%B")]
    format: String,

    #[structopt(name = "source", parse(from_os_str))]
    sources: Vec<PathBuf>,
}

fn process_file(file_path: &PathBuf, root: &Path, dry_run: bool, format: &str) -> Option<PathBuf> {
    let file_date = resolvers::get_date_for_file(file_path);
    if let Some(date) = file_date {
        let out_path = root
            .join(date.format(format).to_string())
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

fn list_directories(directories: &[PathBuf]) -> HashMap<PathBuf, Vec<PathBuf>> {
    let mut directory_map = HashMap::with_capacity(directories.len());
    for directory in directories.iter() {
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

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(100);
    spinner.set_message("Searching for files...");
    let directory_map = list_directories(&opts.sources);
    let file_count = directory_map
        .values()
        .flatten()
        .count()
        .try_into()
        .expect("Too many files");
    spinner.finish();

    let multi_progress = MultiProgress::new();
    let main_progress = multi_progress.add(ProgressBar::new(file_count));
    let error_progress = multi_progress.add(ProgressBar::new(file_count));

    main_progress
        .set_style(ProgressStyle::default_bar().template("{wide_bar:.cyan/blue} {pos}/{len}"));
    error_progress
        .set_style(ProgressStyle::default_bar().template("Errors: {wide_bar:.red} {pos}/{len}"));

    let mut error_files = Vec::new();

    let multi_progress_thread = thread::spawn(move || {
        multi_progress.join().expect("Failed to join");
    });

    for (directory, files) in directory_map.iter() {
        for file in files.iter() {
            let out_path = process_file(file, directory, opts.dry_run, &opts.format);
            match out_path {
                Some(out) => {
                    main_progress.println(format!("{} -> {}", file.display(), out.display()));
                }
                None => {
                    error_progress.println(format!("Failed to parse date for {}", file.display()));
                    error_progress.inc(1);
                    error_files.push(file.to_path_buf());
                }
            }
            main_progress.inc(1);
        }
    }
    main_progress.abandon();
    if error_progress.position() > 0 {
        error_progress.abandon();
    }

    multi_progress_thread
        .join()
        .expect("Multi progress thread panicked");

    if !error_files.is_empty() {
        println!("{} files failed:", error_files.len());
        for error_file in error_files.iter() {
            println!("{}", error_file.display());
        }
    }
}
