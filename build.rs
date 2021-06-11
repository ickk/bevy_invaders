use chrono;
use git2::Repository;
use std::env;

fn main() {
  println!("cargo:rerun-if-changed=.git/index");
  let commit = {
    if let Ok(repo) = Repository::open(env::var("CARGO_MANIFEST_DIR").unwrap()) {
      if let Ok(head) = repo.head() {
        if let Ok(commit) = head.peel_to_commit() {
          let mut s = commit.id().to_string();
          s.push_str(" ");
          s.push_str(commit.summary().unwrap_or(""));
          Some(s)
        } else {None}
      } else {None}
    } else {None}
  };
  if let Some(s) = commit {
    println!("cargo:rustc-env=BUILD_COMMIT={}", s);
  }
  println!("cargo:rustc-env=BUILD_DATE={}", chrono::Local::now().to_rfc2822());
}
