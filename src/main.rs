use std::io;
use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;

fn rfs_dir(dir: PathBuf) -> io::Result<()> {
    let mut fifo = VecDeque::new();
    fifo.push_back(dir);
    while !fifo.is_empty() {
        let current_dir = fifo.pop_front().unwrap();
        if current_dir.is_dir() {

            // TODO: Permissions
            let subdirs = match fs::read_dir(&current_dir) {
                Ok(val) => val,
                Err(_) => continue,
            };
            

            for child_dir in subdirs {
                let dir_entry = match child_dir {
                    Ok(val) => val.path(),
                    Err(_) => continue,
                };
                fifo.push_back(dir_entry);
            }
            println!("{}", current_dir.to_string_lossy());
        }
    }
    Ok(())
}


fn main() {
    rfs_dir(PathBuf::from(".")).unwrap();
}
