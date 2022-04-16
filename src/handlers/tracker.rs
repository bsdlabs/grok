use git2::{Cred, Direction, Error, FetchOptions, RemoteCallbacks};
use serde::{Deserialize, Serialize};
use std::{path::Path};

#[derive(Serialize, Deserialize)]
pub struct Repo {
    pub full_name: String,
    pub name: String,
    pub ssh_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Head {
    pub repo: Repo,
}

#[derive(Serialize, Deserialize)]
pub struct Pr {
    pub url: String,
    pub state: String,
    pub head: Head,
}

#[derive(Serialize, Deserialize)]
pub struct Req {
    pub number: i32,
    pub pull_request: Pr,
}


fn create_callbacks<'a>(path: String) -> RemoteCallbacks<'a> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
        Cred::ssh_key(username_from_url.unwrap(), None, Path::new(&path), None)
    });

    callbacks
}

fn fetch_authenticate(fetch_options: &mut FetchOptions<'_>, path: String) {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, username_from_url, _allowed_types| {
        Cred::ssh_key(username_from_url.unwrap(), None, Path::new(&path), None)
    });

    fetch_options.remote_callbacks(callbacks);
}

pub fn clone_repo(path: &str, full_name: &str, key_path: &str) -> Result<(), Error> {
    let mut fo = git2::FetchOptions::new();
    fetch_authenticate(
        &mut fo,
        key_path.to_string(),
    );

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    builder
        .clone(
            &format!("git@github.com:{}.git", full_name),
            Path::new(path),
        ).unwrap();

    Ok(())
}

pub fn fetch_pr(repo: git2::Repository, _url: &str, pull: &str, key_path: &str) -> Result<(), git2::Error> {
    let mut fo = git2::FetchOptions::new();
    fetch_authenticate(
        &mut fo,
        key_path.to_string(),
    );

    repo.find_remote("origin")?
        .fetch(&[format!("refs/{}:pull-event", pull)], Some(&mut fo), None)
        .unwrap();

    Ok(())
}

pub fn sync_target(repo: git2::Repository, target_key: &str) -> Result<(), git2::Error> {
    let mut po = git2::PushOptions::default();
    let cb = create_callbacks(
        target_key.to_string(),
    );
    po.remote_callbacks(cb);
    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => panic!("remote not found"),
    };

    remote
        .connect_auth(
            Direction::Push,
            Some(create_callbacks(
                target_key.to_string(),
            )),
            None,
        )
        .unwrap();
    remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut po))
}
