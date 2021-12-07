use std::io;
use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;
use regex::Regex;
use structopt::StructOpt;

//fn path_prefix_trim(dir_entry: &str, ignore_regex: &Vec<Regex>) -> String  {
//    lazy_static! {
//        static ref RE: Regex = Regex::new( r"^.*/" ).unwrap();
//    };
//    RE.replace_all(dir_entry, "").to_string()
//}

fn rfs_dir(dir: PathBuf, ignore_regex: &Vec<Regex>) -> io::Result<()> {
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
                let mut add_dir = true;
                for re in ignore_regex {
                    if re.is_match(dir_str) {
                        add_dir = false;
                    }
                }
                if add_dir {
                    fifo.push_back(dir_entry)
                }

            }

            // Print result to stdout
            println!("{}", current_dir.to_string_lossy());
        }
    }
    Ok(())
}

#[derive(Debug, StructOpt)]
struct Opt {
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

    rfs_dir(PathBuf::from("."), &ignore_regex).unwrap();
}
