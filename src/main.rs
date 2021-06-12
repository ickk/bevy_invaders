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
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  let texture_handle = asset_server.load("ferris.png");
  let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(26.0, 26.0), 2, 3);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(SpriteSheetBundle {
    texture_atlas: texture_atlas_handle,
    transform: Transform::from_scale(Vec3::splat(2.0)),
    ..Default::default()
  }).insert(Timer::from_seconds(0.1, true));
}
