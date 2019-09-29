use std::fs::read_dir;
use std::path::PathBuf;
use structopt::StructOpt;
use console::Term;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_str))]
    source_path: PathBuf,
    #[structopt(parse(from_str))]
    target_path: Option<PathBuf>,
}

/// Iterate a directory that has gitignore information (even if that came)
/// from a parent).
fn iter_gi_dir(term: &Term, path: &PathBuf, gi: &gitignore::File) -> Result<(), std::io::Error> {
    for de in read_dir(path)? {
        if let Ok(entry) = de {
            let filename = entry.path(); 
            if gi.is_excluded(&filename).unwrap() || filename.ends_with(".git") {
                println!("{:?} is excluded", filename);
            } else {
                let md = entry.metadata()?;
                if md.is_dir() {
                    println!("{:20} {:>7}", entry.file_name().to_string_lossy(),  "DIR");
                    gi_iterate(term, &entry.path(), Some(gi))?;
                } else {
                    println!("{:20} {:>7}", entry.file_name().to_string_lossy(), md.len());
                }

            }
        };
    }
    Ok(())
}

/// Iterate a directory that has no gitignore information
fn iter_no_gi_dir(term: &Term, path: &PathBuf) -> Result<(), std::io::Error> {
    for de in read_dir(path)? {
        if let Ok(entry) = de {
            let md = entry.metadata()?;
            if md.is_dir() {
                println!("{:20} {:>7}", entry.file_name().to_string_lossy(),  "DIR");
                gi_iterate(term, &entry.path(), None)?;
            } else {
                println!("{:20} {:>7}", entry.file_name().to_string_lossy(), md.len());
            }
        };
    }
    Ok(())
}

/// Iterate a directory tree, searching for and applying `.gitignore` rules
fn gi_iterate(term: &Term, path: &PathBuf, parent_gi: Option<&gitignore::File>) -> Result<(), std::io::Error> {
    // check if the directory has a .gitignore in it
    let gi_path = path.join(".gitignore");
    if gi_path.exists() {
        // Create the gitignore file object, use it to iterate directories and files
        println!("{:?} contains a .gitignore file", path);
        let gi = gitignore::File::new(&gi_path).unwrap();
        iter_gi_dir(term, path, &gi)?;
    } else {
        println!("{:?} has no .gitignore", path);
        if parent_gi.is_some() {
            println!("anscestor has a .gitignore. Using for this directory.");
            iter_gi_dir(term, path, parent_gi.unwrap())?;
        } else {
            iter_no_gi_dir(term, path)?;
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    let term = Term::stdout();
    term.write_line(&format!("{:?}", opt)).unwrap();
    match gi_iterate(&term, &opt.source_path, None) {
        Ok(()) => (),
        Err(err) => {
            term.write_line(&format!("error: {:?}", err)).unwrap();
        }
    }
}
