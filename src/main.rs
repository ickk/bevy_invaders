fn print_build_metadata() {
  if let Some(build_date) = option_env!("BUILD_DATE") {
    eprintln!("Build date: {}", build_date)
  }
  if let Some(build_commit) = option_env!("BUILD_COMMIT") {
    eprintln!("Commit: {}", build_commit)
  }
}

fn main() {
  print_build_metadata();
}
