use git2::{Cred, RemoteCallbacks};
use std::env;
use std::fs::remove_dir_all;
use std::path::Path;

use crate::parser::Git;

pub fn download_git(repo: &Git, id: Option<&str>) {
    let path = match id {
        Some(ssh) => ssh,
        None => "id_rsa",
    };

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/{}", env::var("HOME").unwrap(), path)),
            None,
        )
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    let out = &format!("/tmp/{}", repo.name);
    // Lazy: Delete path then clone again
    if Path::new(out).exists() {
        remove_dir_all(out).unwrap();
    }
    builder.clone(repo.url.as_str(), Path::new(out)).unwrap();
}
