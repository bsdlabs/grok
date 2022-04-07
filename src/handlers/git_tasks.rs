use git2::{Cred, Direction, Error, FetchOptions, RemoteCallbacks};
use std::{env, path::Path};

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

pub fn clone_repo(full_name: &str) -> Result<(), Error> {
    let mut fo = git2::FetchOptions::new();
    fetch_authenticate(
        &mut fo,
        format!(
            "{}/.ssh/{}-id_ed25519",
            env::var("HOME").unwrap(),
            full_name
        )
        .to_string(),
    );

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    builder
        .clone(
            &format!("git@github.com:{}.git", full_name),
            Path::new(&format!("/tmp/{}", full_name)),
        )
        .unwrap();

    Ok(())
}

pub fn fetch_pull(repo: git2::Repository, _url: &str, pull: &str) -> Result<(), git2::Error> {
    let mut fo = git2::FetchOptions::new();
    fetch_authenticate(
        &mut fo,
        format!("{}/.ssh/bsdlabs/ports-id_ed25519", env::var("HOME").unwrap()).to_string(),
    );

    repo.find_remote("origin")?
        .fetch(&[format!("refs/{}:pull-event", pull)], Some(&mut fo), None)
        .unwrap();

    Ok(())
}

pub fn sync_target(repo: git2::Repository) -> Result<(), git2::Error> {
    let mut po = git2::PushOptions::default();
    let cb = create_callbacks(
        format!(
            "{}/.ssh/bsdlabs/freebsd-ports-id_ed25519",
            env::var("HOME").unwrap()
        )
        .to_string(),
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
                "/root/.ssh/bsdlabs/freebsd-ports-id_ed25519".to_string(),
            )),
            None,
        )
        .unwrap();
    remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut po))
}
