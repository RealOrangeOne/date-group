use console::style;
use glob::glob;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{create_dir_all, rename};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::thread;
use structopt::StructOpt;

mod resolvers;
mod utils;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short, long, help = "Don't move files")]
    dry_run: bool,

    #[structopt(long, default_value = "%Y/%B", help = "Format to group files by")]
    format: String,

    #[structopt(name = "source", parse(from_os_str))]
    sources: Vec<PathBuf>,

    #[structopt(short, long)]
    verbose: bool,
}

fn process_file(file_path: &PathBuf, root: &Path, opts: &Opt) -> Option<PathBuf> {
    let file_date = resolvers::get_date_for_file(file_path);
    if let Some(date) = file_date {
        let out_path = root
            .join(date.format(&opts.format).to_string())
            .join(file_path.file_name()?);
        if out_path == file_path.to_path_buf() {
            return Some(out_path);
        }
        if out_path.exists() {
            return None;
        }
        if !opts.dry_run {
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

#[inline]
fn abandon_or_clear(pb: ProgressBar) {
    if pb.position() > 0 {
        pb.abandon();
    } else {
        pb.finish_and_clear();
    }
}

fn main() {
    let opts = Opt::from_args();

    if opts.sources.is_empty() {
        eprintln!("{}", style("At least 1 source must be provided.").red());
        exit(1);
    }

    let spinner = ProgressBar::new_spinner();

    if opts.verbose {
        spinner.enable_steady_tick(100);
        spinner.set_message("Searching for files...");
    }

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
    let in_place_progress = multi_progress.add(ProgressBar::new(file_count));
    let error_progress = multi_progress.add(ProgressBar::new(file_count));

    main_progress.set_style(ProgressStyle::default_bar().template("{wide_bar:.green} {pos}/{len}"));
    in_place_progress
        .set_style(ProgressStyle::default_bar().template("In place: {wide_bar:.cyan} {pos}/{len}"));
    error_progress
        .set_style(ProgressStyle::default_bar().template("Errors: {wide_bar:.red} {pos}/{len}"));

    let multi_progress_thread = thread::spawn(move || {
        multi_progress.join().expect("Failed to join");
    });

    for (directory, files) in directory_map.iter() {
        for file in files.iter() {
            let out_path = process_file(file, directory, &opts);
            match out_path {
                Some(out) => {
                    if out == file.to_path_buf() {
                        if opts.verbose {
                            main_progress.println(
                                style(format!("{} already in place", out.display()))
                                    .cyan()
                                    .to_string(),
                            );
                        }
                        in_place_progress.inc(1);
                    } else if opts.verbose {
                        main_progress.println(
                            style(format!("{} -> {}", file.display(), out.display()))
                                .green()
                                .to_string(),
                        );
                    }
                }
                None => {
                    if opts.verbose {
                        error_progress.println(
                            style(format!("Failed to get date for {}", file.display()))
                                .red()
                                .to_string(),
                        );
                    }
                    error_progress.inc(1);
                }
            }
            main_progress.inc(1);
        }
    }

    main_progress.abandon();
    abandon_or_clear(error_progress);
    abandon_or_clear(in_place_progress);

    multi_progress_thread
        .join()
        .expect("Multi progress thread panicked");
}
