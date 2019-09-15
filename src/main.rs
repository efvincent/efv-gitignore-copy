use std::path::PathBuf;
use std::fs::{read_dir};

fn get_top_dir() -> PathBuf {
    PathBuf::from(".")
}

fn my_dir(path: &PathBuf) -> Result<(), std::io::Error> {
    for entry in read_dir(path)? {
        if let Ok(entry) = entry {
            let md = entry.metadata()?;
            if md.is_dir() {
                println!("{:?} <dir>", entry.file_name());
            } else {
                println!("{:?} {}", entry.file_name(), md.len());
            }
        }
    }
    Ok(())
}

fn main() {
    match my_dir(&get_top_dir()) {
        Ok(()) => (),
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

    // parse arguments
    //  source directory / target directory
    // go to source directory
    // if there's both a .git and a .gitignore, do a git cleanup
    // copy all the files
    // now any undesirable directories are gone, so every remaining directory can be processed the same way 
    // enqueue all the child directories
    // pop the next thing off the queue
    // create the path
    // repeat
}