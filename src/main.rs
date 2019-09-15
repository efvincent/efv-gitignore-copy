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

fn my_dir(path: &PathBuf) -> Result<(), std::io::Error> {
    for entry in read_dir(path)? {
        if let Ok(entry) = entry {
            let md = entry.metadata()?;
            if md.is_dir() {
                println!("{:20} {:>7}", entry.file_name().to_string_lossy(), "DIR");
            } else {
                println!("{:20} {:>7}", entry.file_name().to_string_lossy(), md.len());
            }
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    match my_dir(&opt.source_path) {
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
