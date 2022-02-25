use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;
use regex::Regex;
use structopt::StructOpt;

fn rfs_dir(dir: PathBuf, ignore_regex: &Vec<Regex>) {
    let mut fifo = VecDeque::new();
    fifo.push_back(dir);
    while !fifo.is_empty() {
        let current_dir = fifo.pop_front().unwrap();
        if current_dir.is_dir() {

            // TODO: Print error on permission denied?
            let subdirs = match fs::read_dir(&current_dir) {
                Ok(val) => val,
                Err(_) => continue,
            };

            for child_dir in subdirs {
                let dir_entry = match child_dir {
                    Ok(val) => val.path(),
                    Err(_) => continue,
                };
                
                let dir_str = match dir_entry.to_str() {
                    Some(val) => val,
                    None => continue 
                };

                // Ignore directories from Regex
                let add_dir = !ignore_regex.iter().any(|re|re.is_match(dir_str));
                if add_dir {
                    fifo.push_back(dir_entry)
                }
            }

            // Print result to stdout
            println!("{}", current_dir.to_string_lossy());
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, short, default_value=".")]
    /// Search path (defaults to '.')
    path: PathBuf,

    #[structopt(long, short)]
    /// Path suffixes to ignore
    ignore_paths: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    let mut ignore_regex: Vec<Regex> = Vec::new();
    for entry in opt.ignore_paths {
        let regex_str = format!(r"{}$", entry);
        ignore_regex.push( Regex::new(&regex_str).unwrap() );
    }
    rfs_dir(opt.path, &ignore_regex);
}
