use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResized;


mod tiles_plugin;
use tiles_plugin::*;
    
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
        .run();
}

fn setup(mut commands: Commands, mut update_tiles_writer: EventWriter<UpdateTilesEvent>,) {
    update_tiles_writer.send(UpdateTilesEvent);
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

