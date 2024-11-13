use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::grid_logic_plugin::{PlayerEntityBundle, Direction, OnGrid, DirectionFacing, Location, ObstacleLocation, Offset, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, spawn_player);
    }
}

fn spawn_player(
mut commands: Commands, asset_server: Res<AssetServer>,
    mut setup_reader: EventReader<SetupEvent>,
    ) {
for _event in setup_reader.read() {
    let texture_handle_frog = asset_server.load("player/drowsy_frog_sprite.png");                                                                                     

    commands.spawn((
        SpriteBundle {
            texture: texture_handle_frog.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 15.0),
            ..Default::default()        
        },
        PlayerEntityBundle {
player: Player,
direction_facing: DirectionFacing { facing:Direction::North },
location: Location { grid_x: 5.0, grid_y: 5.0 },
obstacle_location: ObstacleLocation { is_obstacle: true, grid_x: 5.0, grid_y: 5.0 },
offset: Offset { offset: false, off_x: 0.0, off_y: 0.0 },
on_grid: OnGrid,
        }
    ));

}//sync player obstacle with player location!
}
