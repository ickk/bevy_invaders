use bevy::{prelude::*,
           sprite};
use std::collections::HashMap;

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
    .add_system(movement_system.system())
    .add_system(player_sprite_system.system())
    .add_system(player_shoot_system.system())
    .add_system(projectile_move_system.system())
    .add_event::<ShootEvent>()
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
  handles.textures
    .insert("player", asset_server.load("ferris.png"));
  handles.textures
    .insert("projectile", asset_server.load("projectile.png"));
  let projectile_material_handle = materials.add(
    handles.textures.get("projectile").unwrap().clone().into()
  );
  handles.color_materials
    .insert("projectile", projectile_material_handle);

  let mut texture_atlas = TextureAtlas::from_grid(
    handles.textures.get("player").unwrap().clone(),
    Vec2::new(29.0, 21.0), 6, 1);
  texture_atlas.add_texture(sprite::Rect {
    min: Vec2::new(0.0, 0.0),
    max: Vec2::new(29.0, 21.0)
  });
  handles.texture_atlases
    .insert("player", texture_atlases.add(texture_atlas));

  // Make background black
  commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

  // spawn camera
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  // Spawn player
  commands.spawn_bundle(SpriteSheetBundle {
    texture_atlas: handles.texture_atlases.get("player").unwrap().clone(),
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
  mut shootevent: EventReader<ShootEvent>,
) {
  let (mut sprite, velocity, player) = query.single_mut()
    .expect("There was more than one player entity");
  let mut event = None;
  for e in shootevent.iter() {
    event = Some(e);
  }
  match (&player.facing, velocity.x.abs(), event) {
    (Direction::Left, _, Some(e)) => {sprite.index = 0; eprintln!("debug: {:#?}", e)},
    (Direction::Right, _, Some(_)) => sprite.index = 1,
    // (Direction::Left, speed, Some(_)) if speed > 0.0 => sprite.index = 0,
    // (Direction::Right, speed, Some(_)) if speed > 0.0 => sprite.index = 1,
    (Direction::Left, _, None) => sprite.index = 4,
    (Direction::Right, _, None) => sprite.index = 5,
    _ => sprite.index = 4,
  }
}

fn movement_system(
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
    transform.translation.x += velocity.x * 0.5;
    transform.translation.y += velocity.y;
  }
}

// TODO: add timer for shoot sprite
// TODO: alternate shooting claws
fn player_shoot_system(
  mut query: Query<(&Player,
                    &Velocity,
                    &Transform)>,
  mut event: EventWriter<ShootEvent>,
  mut commands: Commands,
  handles: Res<HandleMap>,
  input: Res<Input<KeyCode>>,
) {
  let (player, player_velocity, player_transform) = query.single_mut()
    .expect("There was more than one player entity");

  if input.just_released(KeyCode::Space) {
    let mut projectile_transform = player_transform.clone();
    projectile_transform.translation.y += 30.0;
    match player.facing {
      Direction::Right  => projectile_transform.translation.x += 30.0,
      Direction::Left|_ => projectile_transform.translation.x -= 30.0,
    }

    commands.spawn_bundle(SpriteBundle {
      material: handles.color_materials.get("projectile").unwrap().clone(),
      transform: projectile_transform,
      ..Default::default()
    }).insert(Velocity {
      x: player_velocity.x,
      y: 4.0,
    }).insert(Projectile);

    event.send(ShootEvent)
  }
}

#[derive(Copy, Clone)]
enum Direction {
  Right,
  Left,
  None,
}

struct Projectile;
#[derive(Debug)]
struct ShootEvent;

struct Player {
  facing: Direction,
}

struct Velocity {
  x: f32,
  y: f32,
}
