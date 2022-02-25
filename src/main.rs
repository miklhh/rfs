use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;
use std::sync::Arc;
use std::io::Write;
use tokio::io::ErrorKind::BrokenPipe;
use futures::StreamExt;
use futures::stream::FuturesUnordered;
use tokio::sync::RwLock;
use regex::Regex;
use structopt::StructOpt;

async fn rfs_dir(dir: PathBuf, ignore_regex: Vec<Regex>) {
    let ignore_regex = Arc::new(ignore_regex);
    let fifo = Arc::new(RwLock::new(VecDeque::new()));
    fifo.write().await.push_back(dir);

    let mut worker_handles = FuturesUnordered::new();

    loop {
        let fifo = fifo.clone();
        let ignore_regex = ignore_regex.clone();
        let next = {
            let mut fifo = fifo.write().await;
            fifo.pop_front()
        };

        if let Some(current_dir) = next {
            let handle = tokio::spawn(async move {
                if current_dir.is_dir() {
                    // TODO: Print error on permission denied?
                    let subdirs = match fs::read_dir(&current_dir) {
                        Ok(val) => val,
                        Err(_) => return Ok(()),
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
                            fifo.write().await.push_back(dir_entry);
                        }
                    }

                    // Print result to stdout
                    // println!("{}", current_dir.to_string_lossy());
                    let write_result = writeln!(std::io::stdout(), "{}", current_dir.to_string_lossy());
                    if write_result.is_err() {
                        return write_result;
                    }
                }
                Ok(())
            });
            worker_handles.push(handle);
        }
        else {
            match worker_handles.next().await {
                Some(Ok(Ok(()))) => continue,
                // Break early if the receiver disconnected
                Some(Ok(Err(e))) if e.kind() == BrokenPipe => {
                    break;
                }
                // If we got another io error while printing panic
                Some(Ok(e)) => {e.unwrap()}
                Some(Err(e)) => {panic!("Error while printing {:?}", e)},
                None => {}
            }
            if worker_handles.is_empty() {
                break;
            }
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

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let mut ignore_regex: Vec<Regex> = Vec::new();
    for entry in opt.ignore_paths {
        let regex_str = format!(r"{}$", entry);
        ignore_regex.push( Regex::new(&regex_str).unwrap() );
    }
    rfs_dir(opt.path, ignore_regex).await;
}
