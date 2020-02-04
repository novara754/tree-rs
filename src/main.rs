use std::path::{PathBuf, Path};
use std::{fs, io};
use structopt::StructOpt;
use tree::util::IdentifyLast;

#[derive(StructOpt)]
struct Opts {
    #[structopt(name = "ROOT")]
    root: PathBuf,
}

fn main() {
    let opts = Opts::from_args();
    if !opts.root.is_dir() {
        eprintln!("tree: {} is not a directory.", opts.root.display());
    } else {
        println!("{}", opts.root.display());
        print_dir(opts.root.as_path(), String::from("")).unwrap();
    }
}

/// Recursively traverse directories, printing each entry
/// in a tree-like structure.
fn print_dir(dir: &Path, indent: String) -> io::Result<()> {
    for (is_last, entry) in fs::read_dir(dir)?.identify_last() {
        let path = entry?.path();
        let branch = if is_last {
            "`---"
        } else {
            "|---"
        };
        println!("{}{}{}", indent, branch, path.file_name().unwrap().to_str().unwrap());
        if path.is_dir() {
            let new_indent = if is_last {
                "    "
            } else {
                "|   "
            };
            print_dir(&path, format!("{}{}", indent, new_indent))?;
        }
    }

    Ok(())
}
