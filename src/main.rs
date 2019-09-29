use std::fs::read_dir;
use std::path::PathBuf;
use structopt::StructOpt;
use console::style;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_str))]
    source_path: PathBuf,
    #[structopt(parse(from_str))]
    target_path: Option<PathBuf>,
}

/// Iterate a directory that has gitignore information (even if that came)
/// from a parent).
fn iter_gi_dir(path: &PathBuf, gi: &gitignore::File) -> Result<(), std::io::Error> {
    for de in read_dir(path)? {
        if let Ok(entry) = de {
            let filename = entry.path(); 
            if gi.is_excluded(&filename).unwrap() || filename.ends_with(".git") {
                println!("{:30} {}", style(filename.to_string_lossy()).red(), style("excluded").dim());
            } else {
                let md = entry.metadata()?;
                let filename = entry.file_name();
                let fns = &filename.to_string_lossy();
                if md.is_dir() {
                    println!("{:30} {:>7}", style(fns).blue().bold(),  "DIR");
                    gi_iterate(&entry.path(), Some(gi))?;
                } else {
                    println!("{:30} {:>7}", style(fns).blue(), md.len());
                }

            }
        };
    }
    Ok(())
}



/// Iterate a directory that has no gitignore information
fn iter_no_gi_dir(path: &PathBuf) -> Result<(), std::io::Error> {
    for de in read_dir(path)? {
        if let Ok(entry) = de {
            let md = entry.metadata()?;
            let filename = entry.file_name();
            let fns = &filename.to_string_lossy();
            if md.is_dir() {
                println!("{:30} {:>7}", style(fns).green(),  style("DIR").blue());
                gi_iterate(&entry.path(), None)?;
            } else {
                println!("{:30} {:>7}", style(fns).yellow(), md.len());
            }
        };
    }
    Ok(())
}

/// Iterate a directory tree, searching for and applying `.gitignore` rules
fn gi_iterate(path: &PathBuf, parent_gi: Option<&gitignore::File>) -> Result<(), std::io::Error> {
    // check if the directory has a .gitignore in it
    let gi_path = path.join(".gitignore");
    if gi_path.exists() {
        // Create the gitignore file object, use it to iterate directories and files
        println!("{:?} contains a .gitignore file", path);
        let gi = gitignore::File::new(&gi_path).unwrap();
        iter_gi_dir(path, &gi)?;
    } else {
        println!("{:?} has no .gitignore", path);
        if parent_gi.is_some() {
            println!("anscestor has a .gitignore. Using for this directory.");
            iter_gi_dir(path, parent_gi.unwrap())?;
        } else {
            iter_no_gi_dir(path)?;
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    println!("Options: {}", style(format!("{:?}", opt)).cyan());
    match gi_iterate(&opt.source_path, None) {
        Ok(()) => (),
        Err(err) => {
            println!("error: {}", style(format!("{:?}", err)).red());
        }
    }
}
