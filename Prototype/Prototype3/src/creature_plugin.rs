use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::grid_logic_plugin::{GridEntityBundle, Direction, OnGrid, DirectionFacing, Location, RequestLocation, ObstacleLocation, Offset};

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, spawn_creature);
    }
}

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 9;
const CELL_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct Creature;

fn spawn_creature(
mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Assets<Image>>, 
    mut setup_reader: EventReader<SetupEvent>,
    ) {
for _event in setup_reader.read() {
    let texture_handle_hedgehog = asset_server.load("creatures/hedgehog.png");                                                                                     

    commands.spawn((
        Creature,
        SpriteBundle {
            texture: texture_handle_hedgehog.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 15.0),
            ..Default::default()        
        },
        GridEntityBundle {
direction_facing: DirectionFacing { facing:Direction::North },
location: Location { grid_x: 9.0, grid_y: 7.0 },
request_location: RequestLocation { requesting: false, grid_x: 9.0, grid_y: 7.0 },
obstacle_location: ObstacleLocation { obstacle: true, grid_x: 9.0, grid_y: 7.0 },
offset: Offset { offset: false, off_x: 0.0, off_y: 0.0 },
on_grid: OnGrid,
        }
    ));

}
}
