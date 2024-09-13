use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use tiles::*;
use rand::Rng;
mod tiles;

const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 18;
const F_GRID_WIDTH: f32 = GRID_WIDTH as f32;
const F_GRID_HEIGHT: f32 = GRID_HEIGHT as f32;
const SCREEN_WIDTH: f32 = 320.0;
const SCREEN_HEIGHT: f32 = 180.0;
const CELL_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (320.0, 180.0).into(),
                    resizable: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, resize_camera)
        .add_systems(Update, move_entities)
        .add_systems(Update, update_pos.after(move_entities))
        .add_systems(Update, keyboard_movement)
        .run();
}
#[derive(Component)]
struct Player;

#[derive(Component)]
struct OnMap;

#[derive(Component)]
struct GridPos {
    xgrid: f32,
    ygrid: f32,
}

#[derive(Component)]
struct UpdateGridPos {
    xgrid: f32,
    ygrid: f32,
}

#[derive(Component)]
struct ObstaclePos {
    xgrid: f32,
    ygrid: f32,
}

#[derive(Component)]
struct Health {
    health: i32,
    max_health: i32,
}

#[derive(Component)]
struct Alive {
    remove: bool,
}

#[derive(Component)]
struct AttackImage {
    time_visual: i32,
    remove: bool,
    xgrid: f32,
    ygrid: f32,
}

#[derive(Component)]
struct AttackDamage {
    attack_damage: i32,
    remove: bool,
    xgrid: f32,
    ygrid: f32,
}

#[derive(PartialEq)]
enum Direction {
North, East, South, West, None,
}

#[derive(Component)]
struct PlayerAttackDirection {
    direction:Direction,
}

#[derive(Component)]
struct MovementTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer> /*, mut textures: ResMut<Assets<Image>> not needed currently*/) {
    let texture_handle_grass1 = asset_server.load("grass1.png");
    let texture_handle_grass2 = asset_server.load("grass2.png");
    let texture_handle_grass3 = asset_server.load("grass3.png");
    let texture_handle_grass4 = asset_server.load("grass4.png");
    let texture_handle_water = asset_server.load("water.png");
    let texture_handle_blob = asset_server.load("blobplayer.png");
    let texture_handle_rock = asset_server.load("rock.png");

    commands.spawn((
            Player,
            ObstaclePos { xgrid: 0.0, ygrid: 0.0 },
            OnMap,
            PlayerAttackDirection {direction: Direction::None},
            MovementTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        GridPos { xgrid: 0.0, ygrid: 0.0 },
        UpdateGridPos { xgrid: 0.0, ygrid: 0.0 },
        SpriteBundle {
            texture: texture_handle_blob.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
    ));
fn spawn_rock(
    commands: &mut Commands,
    texture_handle_rock: Handle<Image>,
    spawn_xgrid: f32,
    spawn_ygrid: f32,
    )
    {commands.spawn((
        OnMap,
        GridPos { xgrid: 0.0, ygrid: 0.0 },
        Health { health: 40, max_health: 40},
        Alive { remove: false },
        ObstaclePos { xgrid: 0.0, ygrid: 0.0 },
        UpdateGridPos { xgrid: spawn_xgrid, ygrid: spawn_ygrid },
        SpriteBundle {
            texture: texture_handle_rock.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
    ));}
    spawn_rock(&mut commands, texture_handle_rock.clone(), 1.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 2.0, 5.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 2.0, 7.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 2.0, 9.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 3.0, 1.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 3.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 3.0, 6.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 2.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 4.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 8.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 5.0, 2.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 5.0, 9.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 7.0, 5.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 7.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 8.0, 6.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 9.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 12.0, 7.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 12.0, 9.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 13.0, 1.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 13.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 13.0, 6.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 12.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 14.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 13.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 5.0, 12.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 5.0, 16.0);

// Example 2D array for the grid (0 for empty, 1 for grass)
    let mut rng = rand::thread_rng();
    let mut grid: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [
        [0; GRID_WIDTH]; GRID_HEIGHT];

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            grid[y][x] = rng.gen_range(1..=4); }}

    // Spawn the sprites based on the grid
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
          match grid[y][x] {
                1 => spawn_tile(&mut commands, texture_handle_grass1.clone(), x, y),
                2 => spawn_tile(&mut commands, texture_handle_grass2.clone(), x, y),
                3 => spawn_tile(&mut commands, texture_handle_grass3.clone(), x, y),
                4 => spawn_tile(&mut commands, texture_handle_grass4.clone(), x, y),
                5 => spawn_tile(&mut commands, texture_handle_water.clone(), x, y),
                _ => {}
            }
        }
    }



    commands.spawn(Camera2dBundle::default());
}

fn resize_camera(
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mut projection = query.single_mut();
    let window = windows.single();

    let aspect_ratio = window.width() / window.height();
    let target_aspect_ratio = 320.0 / 180.0;
if aspect_ratio > target_aspect_ratio {
        projection.scaling_mode = ScalingMode::FixedVertical(180.0);
    } else {
        projection.scaling_mode = ScalingMode::FixedHorizontal(320.0);
    }
}

fn move_entities(mut query: Query<(&GridPos, &mut Transform), With<OnMap>>) {
    for (GridPos, mut transform) in &mut query {
       let new_x = (GridPos.xgrid * CELL_SIZE) - (SCREEN_WIDTH / 2.0) + 5.0;
       let new_y = (GridPos.ygrid * CELL_SIZE) - (SCREEN_HEIGHT / 2.0) + 5.0;

        transform.translation.x = new_x;
        transform.translation.y = new_y;
    };
}


fn remove_dead(
    mut commands: Commands, query: Query<(Entity, &Alive)>
    ) {
for (entity, Alive) in query.iter() {
    if Alive.remove {
        commands.entity(entity).despawn();
    }
}
}

//Removes whole attack entity!
fn remove_attack_image(
    mut commands: Commands, query: Query<(Entity, &AttackImage)>
    ) {
for (entity, AttackImage) in query.iter() {
    if AttackImage.remove {
        commands.entity(entity).despawn();
    }
}
}

//Make it remove that component not whole entity!
//Removes only attack damage component
fn remove_attack_damage(
    mut commands: Commands, query: Query<(Entity, &AttackDamage)>
    ) {
for (entity, AttackDamage) in query.iter() {
    if AttackDamage.remove {
        commands.entity(entity).despawn();
    }
}
}
fn health_check(
mut query: Query<(&mut Alive, &Health)>
    ) {
    for (mut Alive, Health) in query.iter_mut() {
        if Health.health <= 0 { Alive.remove = true; }
    }
}

fn apply_damage() {

}

fn create_attack(
    mut query: Query<(&GridPos, &mut PlayerAttackDirection), With<Player>>,
    commands: &mut Commands,
    texture_handle_attack: Handle<Image>,
            ) {
    for (GridPos, mut PlayerAttackDirection) in query.iter_mut() {
   let mut attack_x = GridPos.xgrid; 
   let mut attack_y = GridPos.ygrid; 
   match
       PlayerAttackDirection.direction {
Direction::North => { attack_y += 1.0;}
Direction::South => { attack_y -= 1.0;}
Direction::East => { attack_x += 1.0;}
Direction::West => { attack_x -= 1.0;}
Direction::None => {}
       }
    {commands.spawn((
AttackImage {
    time_visual: 2,
    remove: false,
    xgrid: attack_x,
    ygrid: attack_y,
},

AttackDamage {
    attack_damage: 10,
    remove: false,
    xgrid: attack_x,
    ygrid: attack_y,
},
        SpriteBundle {
            texture: texture_handle_attack.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..Default::default()
        },
    ));}

    //direction enum(change back to None after)
}
}
//Spawn attack image and calc then remove calc once processed
//Make fn for image and calc
fn keyboard_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&GridPos, &mut UpdateGridPos, &mut PlayerAttackDirection), With<Player>>,
    obstacle_query: Query<&ObstaclePos>,
) {
        let mut blocked = false;
        let mut new_x = 0.0;
        let mut new_y = 0.0;
    for (GridPos, mut UpdateGridPos, mut PlayerAttackDirection) in &mut query {
        UpdateGridPos.xgrid = GridPos.xgrid;
        UpdateGridPos.ygrid = GridPos.ygrid;

        if keys.pressed(KeyCode::KeyZ) {
            if keys.just_pressed(KeyCode::ArrowUp) {PlayerAttackDirection.direction = Direction::North}
            else if keys.just_pressed(KeyCode::ArrowDown) {PlayerAttackDirection.direction = Direction::South}
            else if keys.just_pressed(KeyCode::ArrowLeft) {PlayerAttackDirection.direction = Direction::West}
            else if keys.just_pressed(KeyCode::ArrowRight) {PlayerAttackDirection.direction = Direction::East}

        }
        else
            if keys.just_pressed(KeyCode::ArrowUp) {
               if UpdateGridPos.ygrid < F_GRID_HEIGHT - 1.0 {UpdateGridPos.ygrid += 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowDown) {
               if UpdateGridPos.ygrid > 0.0 {UpdateGridPos.ygrid -= 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowLeft) {
               if UpdateGridPos.xgrid > 0.0 {UpdateGridPos.xgrid -= 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowRight) {
               if UpdateGridPos.xgrid < F_GRID_WIDTH - 1.0 {UpdateGridPos.xgrid += 1.0;}
            } 
        new_x = UpdateGridPos.xgrid;
        new_y = UpdateGridPos.ygrid;
            for ObstaclePos in &obstacle_query {
            if  new_x == ObstaclePos.xgrid && new_y ==  ObstaclePos.ygrid {
                blocked = true;
                break;
        }
    }

        // If blocked, update the position to previous position
        if blocked {
            UpdateGridPos.xgrid = GridPos.xgrid;
            UpdateGridPos.ygrid = GridPos.ygrid;
        }
            }

}
//Potential problem! Later when entities don't have a ObstaclePos this won't work
//Perhaps make fn with without ObstaclePos
fn update_pos(
    mut query: Query<(&mut GridPos, &mut ObstaclePos, &UpdateGridPos)>,
    ) {
    for (mut GridPos, mut ObstaclePos, UpdateGridPos) in &mut query {
        GridPos.xgrid = UpdateGridPos.xgrid;
        GridPos.ygrid = UpdateGridPos.ygrid;
        ObstaclePos.xgrid = GridPos.xgrid;
        ObstaclePos.ygrid = GridPos.ygrid;
            }

}
/*
Referrence for potential time components
fn keyboard_movement(
    time: Res<Time>,
    mut query: Query<(&mut MovementTimer)>
    ) {
    for (mut timer) in &mut query {
        // Update the timer
        timer.0.tick(time.delta());

        // Only move the player if the timer has finished
        if timer.0.finished() {
             // Restart the timer
            timer.0.reset();
        }
    }
}
*/
