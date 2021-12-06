use std::io;
use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;
use regex::Regex;

fn path_prefix_trim(dir_entry: &str) -> String  {
    let re = Regex::new( r"^.*/" ).unwrap();
    re.replace_all(dir_entry, "").to_string()
}

fn rfs_dir(dir: PathBuf) -> io::Result<()> {
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

                let trimmed_dir_str = path_prefix_trim(dir_str);
                if trimmed_dir_str != ".git" {
                    fifo.push_back(dir_entry);
                }
            }
            println!("{}", current_dir.to_string_lossy());
        }
    }
    Ok(())
}

fn main() {
    rfs_dir(PathBuf::from(".")).unwrap();
}
