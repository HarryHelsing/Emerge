use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResized;


//add new plugins when ready
mod tiles_plugin;
use tiles_plugin::*;
mod turn_plugin;
use turn_plugin::*;
mod animate_plugin;
use animate_plugin::*;
mod player_plugin;
use player_plugin::*;
mod creature_plugin;
use creature_plugin::*;
mod input_plugin;
use input_plugin::*;
mod obstacle_plugin;
use obstacle_plugin::*;
mod grid_logic_plugin;
use grid_logic_plugin::*;
    
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1920.0, 1080.0).into(),
                    resizable: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, resize_camera)//done
        .add_plugins(tiles_plugin::TilesPlugin)
        .add_plugins(turn_plugin::TurnPlugin)
        .add_plugins(input_plugin::InputPlugin)
        .add_plugins(player_plugin::PlayerPlugin)
        .add_plugins(creature_plugin::CreaturePlugin)
        .add_plugins(animate_plugin::AnimatePlugin)
        .add_plugins(obstacle_plugin::ObstaclePlugin)
        .add_plugins(grid_logic_plugin::GridLogicPlugin)
        .run();//add new plugins when ready
}

fn setup(
    mut commands: Commands, mut setup_writer: EventWriter<SetupEvent>,
    ) {
    setup_writer.send(SetupEvent);
    commands.spawn(Camera2dBundle::default());
}

fn resize_camera(
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    resize_events: EventReader<WindowResized>,
) {
    if !resize_events.is_empty() {
    let mut projection = query.single_mut();
    let window = windows.single();

    let aspect_ratio = window.width() / window.height();
    let target_aspect_ratio = 1920.0 / 1080.0;
if aspect_ratio > target_aspect_ratio {
        projection.scaling_mode = ScalingMode::FixedVertical(1080.0);
    } else {
        projection.scaling_mode = ScalingMode::FixedHorizontal(1920.0);
    }
    }
}

