use std::fs::read_dir;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_str))]
    source_path: PathBuf,
    #[structopt(parse(from_str))]
    target_path: Option<PathBuf>,
}

fn iter_gi_dir(path: &PathBuf, gi: &gitignore::File) -> Result<(), std::io::Error> {
    for de in read_dir(path)? {
        if let Ok(entry) = de {
            let filename = entry.path(); 
            if gi.is_excluded(&filename).unwrap() {
                println!("{:?} is excluded", filename);
            } else {
                let md = entry.metadata()?;
                if md.is_dir() {
                    println!("{:20} {:>7}", entry.file_name().to_string_lossy(),  "DIR");
                    my_dir(&entry.path(), Some(gi))?;
                }  
            }
        };
    }
    Ok(())
}
    
    // for entry in read_dir(path)? {
    //     if let Ok(entry) = entry {
    //         let md = entry.metadata()?;
    //         if md.is_dir() {
    //             println!("{:20} {:>7}", entry.file_name().to_string_lossy(), "DIR");
    //         } else {
    //             println!("{:20} {:>7}", entry.file_name().to_string_lossy(), md.len());
    //         }
    //     }
    // }
fn iter_no_gi_dir(path: &PathBuf) -> Result<(), std::io::Error> {
    for de in read_dir(path)? {
        if let Ok(entry) = de {
            let md = entry.metadata()?;
            if md.is_dir() {
                println!("{:20} {:>7}", entry.file_name().to_string_lossy(),  "DIR");
                my_dir(&entry.path(), None)?;
            }  
        };
    }
    Ok(())
}

fn my_dir(path: &PathBuf, parent_gi: Option<&gitignore::File>) -> Result<(), std::io::Error> {
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
    println!("{:?}", opt);
    match my_dir(&opt.source_path, None) {
        Ok(()) => (),
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

    // go to source directory
    // if there's both a .git and a .gitignore, do a git cleanup
    // copy all the files
    // now any undesirable directories are gone, so every remaining directory can be processed the same way
    // enqueue all the child directories
    // pop the next thing off the queue
    // create the path
    // repeat
}
