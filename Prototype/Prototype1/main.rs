use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use tiles::*;
use rand::Rng;
mod tiles;

const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 18;
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
struct MovementTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer> /*, mut textures: ResMut<Assets<Image>> not needed currently*/) {
    let texture_handle_grass1 = asset_server.load("grass1.png");
    let texture_handle_grass2 = asset_server.load("grass2.png");
    let texture_handle_grass3 = asset_server.load("grass3.png");
    let texture_handle_grass4 = asset_server.load("grass4.png");
    let texture_handle_water = asset_server.load("water.png");
    let texture_handle_blob = asset_server.load("blobplayer.png");

    commands.spawn((
            OnMap,
            Player,
            MovementTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        GridPos { xgrid: 0.0, ygrid: 0.0 },
        SpriteBundle {
            texture: texture_handle_blob.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
    ));

// Example 2D array for the grid (0 for empty, 1 for grass)
    let mut rng = rand::thread_rng();
    let mut grid: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [
        [0; GRID_WIDTH]; GRID_HEIGHT
    ];

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            grid[y][x] = rng.gen_range(1..=4); // Generate a random value between 0 and 2
        }
    }

    // Spawn the sprites based on the grid
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
          match grid[y][x] {
                1 => spawn_tile(&mut commands, texture_handle_grass1.clone(), x, y),
                2 => spawn_tile(&mut commands, texture_handle_grass2.clone(), x, y),
                3 => spawn_tile(&mut commands, texture_handle_grass3.clone(), x, y),
                4 => spawn_tile(&mut commands, texture_handle_grass4.clone(), x, y),
                5 => spawn_tile(&mut commands, texture_handle_water.clone(), x, y),
                _ => {} // Handle empty or other tiles if needed
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

/*
fn keyboard_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut MovementTimer, &mut GridPos), With<OnMap>>
    ) {
    for (mut timer, mut GridPos) in &mut query {
        // Update the timer
        timer.0.tick(time.delta());

        // Only move the player if the timer has finished
        if timer.0.finished() {
            if keys.pressed(KeyCode::ArrowLeft) {
                GridPos.xgrid -= 1.0;
            } else if keys.pressed(KeyCode::ArrowRight) {
                GridPos.xgrid += 1.0;
            } else if keys.pressed(KeyCode::ArrowUp) {
                GridPos.ygrid += 1.0;
            } else if keys.pressed(KeyCode::ArrowDown) {
                GridPos.ygrid -= 1.0;
            }

            // Restart the timer
            timer.0.reset();
        }
    }
}
*/
fn keyboard_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut GridPos, With<OnMap>>
    ) {
    for (mut GridPos) in &mut query {
            if keys.just_pressed(KeyCode::ArrowLeft) {
                GridPos.xgrid -= 1.0;
            } else if keys.just_pressed(KeyCode::ArrowRight) {
                GridPos.xgrid += 1.0;
            } else if keys.just_pressed(KeyCode::ArrowUp) {
                GridPos.ygrid += 1.0;
            } else if keys.just_pressed(KeyCode::ArrowDown) {
                GridPos.ygrid -= 1.0;
            }
    }
}
