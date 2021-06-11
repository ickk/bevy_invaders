use bevy::prelude::*;

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

  App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .run();
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
