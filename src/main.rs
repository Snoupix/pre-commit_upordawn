use std::path::Path;
use std::process::Command;

use git2::{Repository, StatusOptions, StatusShow};

fn main() {
    run_fix("front");
    run_fix("back");
    run_fix("mobile");

    let repo = Repository::discover(Path::new(".")).expect("Cannot find git repo");
    let mut status_options = StatusOptions::new();
    status_options.show(StatusShow::Workdir);
    let statuses = repo.statuses(Some(&mut status_options)).unwrap();

    if !statuses.is_empty() {
        eprintln!(
            "There are {} modified files by the prettier not added to the commit; commit aborted. (to deny this hook/check, use --no-verify)",
            statuses.iter().count()
        );
        std::process::exit(1);
    }
}

fn run_fix(folder: &str) {
    let path = format!("./{}", folder);
    let folder_path = Path::new(&path);

    if !folder_path.join("package.json").exists() {
        eprintln!("package.json not found in {} project", folder);
        std::process::exit(1);
    }

    let output = Command::new("npm")
        .args(["run", "fix"])
        .current_dir(folder_path)
        .output()
        .or_else(|_| {
            let user_cmd = Command::new("whoami").output();

            let user = match user_cmd {
                Ok(out) => out.stdout.iter().map(|&c| c as char).collect::<String>(),
                Err(_) => std::env::var("HOME")
                    .expect("Failed to execute whoami command and failed to get HOME env variable"),
            };

            let fallback_npm = format!(
                "C:\\Users\\{}\\AppData\\Roaming\\npm\\npm.cmd",
                user.split('\\').last().unwrap().trim()
            );

            Command::new(fallback_npm)
                .args(["run", "fix"])
                .current_dir(folder_path)
                .output()
        })
        .expect("Failed to spawn npm fix command");

    if !output.status.success() {
        eprintln!("Failed to spawn npm fix command: {:?}", output);
        std::process::exit(1);
    }

    println!("[{}: npm run fix] Files linted !", folder);
}
