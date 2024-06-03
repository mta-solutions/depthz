use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use crate::parser::Git;

pub fn download_git(repo: &Git) {
    let out = &format!("/tmp/{}", repo.name);
    let output = if Path::new(out).exists() {
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
