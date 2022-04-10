use actix_web::{web, HttpResponse, Responder, Result};
use std::process::Command;

use git2::Repository;

mod tracker;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

fn fetch_pull_event(repo: &git2::Repository) -> Result<(), git2::Error> {
    repo.remote("source", "/tmp/bsdlabs/ports").unwrap();
    repo.find_remote("source")?
        .fetch(&["pull-event:pull-event"], None, None)
}

pub async fn event(info: web::Json<tracker::Req>) -> impl Responder {
    println!("cloning source");
    tracker::clone_repo(&info.pull_request.head.repo.full_name).unwrap();

    println!("cloning target");
    tracker::clone_repo("bsdlabs/freebsd-ports").unwrap();

    println!("fetch pull request on source");
    let source = Repository::open("/tmp/bsdlabs/ports").unwrap();
    tracker::fetch_pr(
        source,
        &info.pull_request.head.repo.ssh_url,
        &format!("pull/{}/head", info.number),
    )
    .unwrap();

    println!("fetch pull request on target (locally from source");
    let target = Repository::open("/tmp/bsdlabs/freebsd-ports").unwrap();
    fetch_pull_event(&target).unwrap();

    println!("merge pull event on main");
    let cmd = Command::new("sh")
        .arg("-c")
        .arg("cd /tmp/bsdlabs/freebsd-ports && git merge --no-edit pull-event")
        .status()
        .expect("cannot execute the merge script");

    if cmd.success() {
        println!("successfully merged");
    } else {
        panic!("cannot merge")
    };

    println!("push to target");
    tracker::sync_target(target).unwrap();

    HttpResponse::Ok()
}
