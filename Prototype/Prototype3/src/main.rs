use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResized;
use crate::attack_plugin::ChangePlayerAttackEvent;


//add new plugins when ready
mod tiles_plugin;
use tiles_plugin::*;
mod animate_plugin;
mod plugin_combat;
use plugin_combat::*;
mod plugin_core;
use plugin_core::*;
mod plugin_entities;
use plugin_entities::*;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;
pub const GRID_WIDTH: usize = 15;
pub const GRID_HEIGHT: usize = 9;
pub const CELL_SIZE: f32 = 128.0;   

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
        .add_plugins(attack_plugin::AttackPlugin)
        .add_plugins(tiles_plugin::TilesPlugin)
        .add_plugins(turn_plugin::TurnPlugin)
        .add_plugins(input_plugin::InputPlugin)
        .add_plugins(player_plugin::PlayerPlugin)
        .add_plugins(creature_plugin::CreaturePlugin)
        .add_plugins(animate_plugin::AnimatePlugin)
        .add_plugins(obstacle_plugin::ObstaclePlugin)
        .add_plugins(grid_logic_plugin::GridLogicPlugin)
        .add_plugins(plugin_combat::health_plugin::HealthPlugin)
        .run();//add new plugins when ready
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut setup_writer: EventWriter<SetupEvent>,
    mut attack_writer: EventWriter<ChangePlayerAttackEvent>,
    ) {
    setup_writer.send(SetupEvent);
    let default_frog_attack = asset_server.load("player/frog_attack.png");
    attack_writer.send(ChangePlayerAttackEvent {
        image_handle: default_frog_attack,
        damage: 20,
        range: 2.0,
    });
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

