use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Instant,
                Duration};

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
    .insert_resource(HandleMap::new())
    .add_startup_system(setup.system())
    .add_system(player_movement_system.system())
    .add_system(player_sprite_system.system())
    .add_system(player_shoot_system.system())
    .add_system(projectile_move_system.system())
    .add_system(enemy_movement_system.system())
    .add_system(enemy_sprite_system.system())
    // TODO: 1-frame lag between enemy movement system and enemy sprite system
    .add_system(collision_system.system())
    .run();
}

struct HandleMap {
  textures: HashMap<&'static str, Handle<Texture>>,
  texture_atlases: HashMap<&'static str, Handle<TextureAtlas>>,
  color_materials: HashMap<&'static str, Handle<ColorMaterial>>,
}
impl HandleMap {
  fn new() -> Self {
    HandleMap {
      textures: HashMap::new(),
      texture_atlases: HashMap::new(),
      color_materials: HashMap::new(),
    }
  }
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut handles: ResMut<HandleMap>,
) {
  // Get handles for assets
  let player_texture_atlas = {
    let texture = asset_server.load("ferris.png");
    handles.textures.insert("player", texture.clone_weak());
    let atlas = texture_atlases.add(TextureAtlas::from_grid(
      texture, Vec2::new(29.0, 21.0), 6, 1));
    handles.texture_atlases.insert("player", atlas.clone_weak());
    atlas
  };
  let _enemy_texture_atlas = {
    let texture = asset_server.load("bird.png");
    handles.textures.insert("enemy", texture.clone_weak());
    let atlas = texture_atlases.add(TextureAtlas::from_grid(
      texture, Vec2::new(29.0, 18.0), 8, 1));
    handles.texture_atlases.insert("enemy", atlas.clone_weak());
    atlas
  };
  let _projectile_material = {
    let texture = asset_server.load("projectile.png");
    handles.textures.insert("projectile", texture.clone_weak());
    let material = materials.add(texture.into());
    handles.color_materials.insert("projectile", material.clone());
    material
  };

  // Make background black
  commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

  // spawn camera
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  // Spawn player
  commands.spawn_bundle(SpriteSheetBundle {
    texture_atlas: player_texture_atlas,
    transform: {
        let mut trans = Transform::from_scale(Vec3::splat(2.0));
        trans.translation.y -= 325.0;
        trans
    },
    ..Default::default()
  }).insert(Player {
    facing: Direction::Left,
    shooting: None,
  }).insert(Velocity {
    x: 0.0,
    y: 0.0,
  });

  // Spawn enemy
  commands.spawn_bundle(SpriteSheetBundle {
    texture_atlas: _enemy_texture_atlas,
    transform: {
      let mut trans = Transform::from_scale(Vec3::splat(2.0));
      trans.translation.y += 325.0;
      trans
    },
    ..Default::default()
  }).insert(Enemy {
    facing: Direction::Right,
  }).insert(Velocity {
    x: 1.0,
    y: 5.0,
  }).insert(Timer::from_seconds(0.2, true));
}

fn collision_system(
  query_projectile: Query<(Entity,
                           &Transform),
                          With<Projectile>>,
  query_enemy: Query<(Entity,
                      &Transform),
                     With<Enemy>>,
  mut commands: Commands,
) {
  for (projectile, projectile_transform) in query_projectile.iter() {
    for (enemy, enemy_transform) in query_enemy.iter() {
      if projectile_transform.translation.x < enemy_transform.translation.x + 20.0
      && projectile_transform.translation.x > enemy_transform.translation.x - 20.0
      && projectile_transform.translation.y < enemy_transform.translation.y + 20.0
      && projectile_transform.translation.y > enemy_transform.translation.y - 20.0 {
        commands.entity(enemy).despawn();
        commands.entity(projectile).despawn();
      }
    }
  }
}

fn enemy_movement_system(
  mut query: Query<(&mut Enemy,
                    &mut Transform,
                    &mut Velocity)>
) {
  for (mut enemy, mut transform, mut velocity) in query.iter_mut() {
    transform.translation.x += velocity.x;

    if transform.translation.x > 600.0
    || transform.translation.x < -600.0 {
      velocity.x *= -1.0;
      transform.translation.y -= velocity.y;
      enemy.facing = match enemy.facing {
        Direction::Left  => Direction::Right,
        Direction::Right => Direction::Left,
      };
    }
  }
}

fn enemy_sprite_system(
  time: Res<Time>,
  mut query: Query<(&mut Timer,
                    &mut TextureAtlasSprite,
                    &Enemy)>,
) {
  for (mut timer, mut sprite, enemy) in query.iter_mut() {
    timer.tick(time.delta());
    if timer.finished() {
      let offset = match enemy.facing {
        Direction::Left  => 4,
        Direction::Right => 0,
      };
      sprite.index = ((sprite.index as usize + 1) % 4 + offset) as u32;
    }
  }
}

fn player_sprite_system(
  mut query: Query<(&mut TextureAtlasSprite,
                    &Velocity,
                    &mut Player)>,
) {
  let (mut sprite, _velocity, mut player) = query.single_mut()
    .expect("There was more than one player entity");

  if let Some(instant) = player.shooting {
    if instant.elapsed() >= Duration::from_millis(100) {
      player.shooting = None
    }
  }

  match (&player.facing, &player.shooting) {
    (Direction::Left, Some(_)) => sprite.index = 0,
    (Direction::Right, Some(_)) => sprite.index = 1,
    (Direction::Left, None) => sprite.index = 4,
    (Direction::Right, None) => sprite.index = 5,
  }
}

fn player_movement_system(
  mut query: Query<(&mut Player,
                    &mut Transform,
                    &mut Velocity)>,
  input: Res<Input<KeyCode>>,
) {
  let (mut player, mut transform, mut velocity) = query.single_mut()
    .expect("There was more than one player entity");

  // update velocity
  // TODO: use a curve for velocity -> linear accel to some maximum velocity, damper
  velocity.x = (input.pressed(KeyCode::Right) as i32 - input.pressed(KeyCode::Left) as i32) as f32 * 4.0;
  // move player
  transform.translation.x = (-550.0 as f32).max(transform.translation.x + velocity.x).min(550.0);
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

fn projectile_move_system(
  mut query: Query<(Entity,
                    &Velocity,
                    &mut Transform),
                   With<Projectile>>,
  mut commands: Commands,
) {
  for (projectile, velocity, mut transform) in query.iter_mut() {
    // despawn projectiles that go off-screen
    if transform.translation.y > 360.0 {
      commands.entity(projectile).despawn();
      continue
    }
    transform.translation.x += velocity.x;
    transform.translation.y += velocity.y;
  }
}

// TODO: add timer for shoot sprite
// TODO: alternate shooting claws
fn player_shoot_system(
  mut query: Query<(&mut Player,
                    &Velocity,
                    &Transform)>,
  mut commands: Commands,
  handles: Res<HandleMap>,
  input: Res<Input<KeyCode>>,
) {
  let (mut player, player_velocity, player_transform) = query.single_mut()
    .expect("There was more than one player entity");

  if input.just_pressed(KeyCode::Space) {
    let mut projectile_transform = player_transform.clone();
    projectile_transform.translation.y += 30.0;
    match player.facing {
      Direction::Right  => projectile_transform.translation.x += 26.0,
      Direction::Left   => projectile_transform.translation.x -= 26.0,
    }

    commands.spawn_bundle(SpriteBundle {
      material: handles.color_materials.get("projectile").unwrap().clone(),
      transform: projectile_transform,
      ..Default::default()
    }).insert(Velocity {
      x: player_velocity.x * 0.08,
      y: 4.0,
    }).insert(Projectile);

    player.shooting = Some(Instant::now());
  }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
  Right,
  Left
}

struct Projectile;
#[derive(Debug)]

struct Player {
  facing: Direction,
  shooting: Option<Instant>,
}

struct Enemy {
  facing: Direction,
}

struct Velocity {
  x: f32,
  y: f32,
}
