use std::path::Path;
use std::process::Command;

use git2::{Repository, StatusOptions, StatusShow};

fn main() {
    run_fix("front");
    run_fix("back");

    let repo = Repository::discover(Path::new(".")).expect("Cannot find git repo");
    let mut status_options = StatusOptions::new();
    status_options.show(StatusShow::Workdir);
    let statuses = repo.statuses(Some(&mut status_options)).unwrap();

    if !statuses.is_empty() {
        eprintln!(
            "There are {} modified files not added to commit, commit aborted. (to deny this check, use --no-verify)",
            statuses.iter().count()
        );
        std::process::exit(1);
    }
}

fn run_fix(folder: &str) {
    let path = format!("./{}", folder);
    let folder_path = Path::new(&path);
    let package_json = folder_path.join("package.json");

    if !package_json.exists() {
        eprintln!("package.json not found in {} project", folder);
        std::process::exit(1);
    }

    let output = Command::new("npm")
        .args(["run", "fix"])
        .current_dir(folder_path)
        .output()
        .expect("Failed to spawn npm fix command");

    if !output.status.success() {
        eprintln!("Failed to spawn npm fix command: {:?}", output);
        std::process::exit(1);
    }

    println!("[{}: npm run fix] Files linted !", folder);
}
