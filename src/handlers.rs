use actix_web::{web, HttpResponse, Responder, Result};
use std::process::Command;
use crate::config;

use git2::Repository;

mod tracker;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

fn fetch_pull_event(repo: &git2::Repository, source_dir: &str) -> Result<(), git2::Error> {
    repo.remote("source", source_dir).unwrap();
    repo.find_remote("source")?
        .fetch(&["pull-event:pull-event"], None, None)
}

pub async fn event(info: web::Json<tracker::Req>) -> impl Responder {
    let config = config::read_config().unwrap();

    let source_list = config.repositories.source;
    let target_list = config.repositories.target;
    let source_repo = &info.pull_request.head.repo.full_name;
    let mut source_key:  Option<&str> = None;
    let mut target_repo: Option<&str> = None;
    let mut target_key: Option<&str> = None;
    for (src, dst) in source_list.iter().zip(target_list.iter()) {
        if src.repo.as_str() == source_repo {
            target_repo = Some(dst.repo.as_str());
            target_key  = Some(dst.key.as_str());
            source_key  = Some(src.key.as_str());
            break;
        }
    }
    let target_repo = target_repo.expect("Config: target repo not found");
    let target_key  = target_key.expect("Config: target key not found");
    let source_key  = source_key.expect("Config: source key not found");


    let dirname =
        format!("/tmp/{}-{}/", &info.pull_request.head.repo.name, info.number);
    let dir = std::path::Path::new(dirname.as_str());
    if dir.exists() {
        std::fs::remove_dir_all(dir).unwrap();
        std::fs::create_dir(dir).unwrap();
    } else {
        std::fs::create_dir(dir).unwrap();
    }
    let sdir = format!("{}{}", dir.display(), "source");
    let tdir = format!("{}{}", dir.display(), "target");
    let sdir = sdir.as_str();
    let tdir = tdir.as_str();

    println!("cloning source");
    tracker::clone_repo(sdir, source_repo, source_key).unwrap();

    println!("cloning target");
    tracker::clone_repo(tdir, target_repo, target_key).unwrap();

    println!("fetch pull request on source");
    let source = Repository::open(sdir).unwrap();
    tracker::fetch_pr(
        source,
        &info.pull_request.head.repo.ssh_url,
        &format!("pull/{}/head", info.number),
        source_key
    )
    .unwrap();

    println!("fetch pull request on target (locally from source");
    let target = Repository::open(tdir).unwrap();
    fetch_pull_event(&target, sdir).unwrap();

    println!("merge pull event on main");
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && git merge --no-edit pull-event", tdir))
        .status()
        .expect("cannot execute the merge script");

    if cmd.success() {
        println!("successfully merged");
    } else {
        std::fs::remove_dir_all(dir).unwrap();
        panic!("cannot merge")
    };

    println!("push to target");
    tracker::sync_target(target, target_key).unwrap();
    std::fs::remove_dir_all(dir).unwrap();

    HttpResponse::Ok()
}
