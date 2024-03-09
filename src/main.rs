use clap::Parser;
use std::process::Command;
use walkdir::WalkDir;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The root directory
    #[arg(short, long)]
    dir: String,
}

fn main() {
    let args = Args::parse();

    println!("Finding all Nargo.toml files below {}", args.dir);

    let nargo_projects = find_nargo_projects(&args.dir);
    for dir in nargo_projects {
        println!("Formatting {}", dir);
        format_dir(&dir);
    }
}

fn find_nargo_projects(dir: &str) -> Vec<String> {
    let mut nargo_project_dirs = vec![];
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.file_name() == "Nargo.toml" {
            // add the parent directory to the list
            nargo_project_dirs.push(entry.path().parent().unwrap().to_str().unwrap().to_string());
        }
    }
    nargo_project_dirs
}

fn format_dir(dir: &str) {
    // run shell script to cd to dir and run `nargo fmt`
    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && nargo fmt", dir))
        .output()
        .expect("failed to execute process");
}
