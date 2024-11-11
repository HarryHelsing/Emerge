use bevy::prelude::*;
use crate::tiles_plugin::UpdateTilesEvent;
use crate::grid_logic_plugin::{GridEntityBundle, Direction, OnGrid, DirectionFacing, Location, RequestLocation, ObstacleLocation, Offset};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, spawn_player);
    }
}

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 9;
const CELL_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct Player;

fn spawn_player(
mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Assets<Image>>, 
    mut update_tiles_reader: EventReader<UpdateTilesEvent>,
    ) {
for _event in update_tiles_reader.read() {
    let texture_handle_frog = asset_server.load("player/drowsy_frog_sprite.png");                                                                                     

    commands.spawn((
        Player,
        SpriteBundle {
            texture: texture_handle_frog.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 15.0),
            ..Default::default()        
        },
        GridEntityBundle {
direction_facing: DirectionFacing { facing:Direction::North },
location: Location { grid_x: 5.0, grid_y: 5.0 },
request_location: RequestLocation { requesting: false, grid_x: 5.0, grid_y: 5.0 },
obstacle_location: ObstacleLocation { obstacle: true, grid_x: 5.0, grid_y: 5.0 },
offset: Offset { offset: false, off_x: 0.0, off_y: 0.0 },
on_grid: OnGrid,
        }
    ));

}
}
