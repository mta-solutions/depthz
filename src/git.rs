use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use crate::parser::Git;

pub fn download_git(repo: &Git) {
    // All relevant repos get stored in the OS's temp directory
    let mut out = std::env::temp_dir();
    out.push(repo.name.clone());
    let output = if Path::new(out.as_path()).exists() {
        Command::new("git")
            .current_dir(out)
            .arg("pull")
            .output()
            .expect("failed to update repo")
    } else {
        Command::new("git")
            .arg("clone")
            .arg(repo.url.clone())
            .arg(out)
            .output()
            .expect("failed to clone repo")
    };

    println!("status - {}: {}", repo.url, output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
