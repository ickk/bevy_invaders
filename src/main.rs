use bevy::{prelude::*,
           sprite};

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
    .add_system(movement_system.system())
    .add_system(player_sprite_system.system())
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
  let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(29.0, 21.0), 6, 1);
  texture_atlas.add_texture(sprite::Rect {
    min: Vec2::new(0.0, 0.0),
    max: Vec2::new(29.0, 21.0)
  });
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  // spawn camera
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  // Spawn player
  commands.spawn_bundle(SpriteSheetBundle {
    texture_atlas: texture_atlas_handle,
    transform: {
        let mut trans = Transform::from_scale(Vec3::splat(2.0));
        trans.translation.y -= 325.0;
        trans
    },
    ..Default::default()
  })
  .insert(Player {
    facing: Direction::None,
  })
  .insert(Velocity {
    x: 0.0,
    y: 0.0,
  });
}

fn player_sprite_system(
  mut query: Query<(&mut TextureAtlasSprite,
                    &Velocity,
                    &Player)>,
) {
  let (mut sprite, velocity, player) = query.single_mut()
    .expect("There was more than one player entity");
  match (&player.facing, velocity.x.abs()) {
    (Direction::Left, speed) if speed > 0.0 => sprite.index = 0,
    (Direction::Right, speed) if speed > 0.0 => sprite.index = 1,
    (Direction::Left, _) => sprite.index = 4,
    (Direction::Right, _) => sprite.index = 5,
    _ => sprite.index = 4,
  }
}

fn movement_system(
  mut query: Query<(&mut Transform,
                    &mut Velocity,
                    &mut Player)>,
  input: Res<Input<KeyCode>>,
) {
  let (mut transform, mut velocity, mut player) = query.single_mut()
    .expect("There was more than one player entity");

  // update velocity
  velocity.x = (input.pressed(KeyCode::Right) as i32 - input.pressed(KeyCode::Left) as i32) as f32 * 4.0;
  // move player
  transform.translation.x = (-600.0 as f32).max(transform.translation.x + velocity.x).min(600.0);
  // update player's direction
  if let Some(direction) = match velocity.x {
    v if v < 0.0 => Some(Direction::Left),
    v if v > 0.0 => Some(Direction::Right),
    _else => {
      if input.just_pressed(KeyCode::Left) {
        Some(Direction::Left)
      } else if input.just_pressed(KeyCode::Right) {
        Some(Direction::Right)
      } else {
        None
      }
    }
  } {
    player.facing = direction
  };
}

#[derive(Copy, Clone)]
enum Direction {
  Right,
  Left,
  None,
}

struct Player {
  facing: Direction,
}

struct Velocity {
  x: f32,
  #[allow(unused)]
  y: f32,
}


