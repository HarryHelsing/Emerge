use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::turn_plugin::GlobalMoveEvent;
use crate::grid_logic_plugin::{CreatureEntityBundle, Direction, OnGrid, DirectionFacing, Location, RequestLocation, ObstacleLocation, Offset, Creature, Player};

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, spawn_creature);
        app.add_systems(Update, move_creature);
    }
}

fn spawn_creature(
mut commands: Commands, asset_server: Res<AssetServer>,
    mut setup_reader: EventReader<SetupEvent>,
    ) {
for _event in setup_reader.read() {
    let texture_handle_hedgehog = asset_server.load("creatures/hedgehog.png");                                                                                     

    commands.spawn((
        SpriteBundle {
            texture: texture_handle_hedgehog.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 15.0),
            ..Default::default()        
        },
        CreatureEntityBundle {
creature: Creature,
direction_facing: DirectionFacing { facing:Direction::North },
location: Location { grid_x: 9.0, grid_y: 7.0 },
request_location: RequestLocation { requesting: false, can_move: true, grid_x: 9.0, grid_y: 7.0 },
obstacle_location: ObstacleLocation { is_obstacle: true, grid_x: 9.0, grid_y: 7.0 },
offset: Offset { offset: false, off_x: 0.0, off_y: 0.0 },
on_grid: OnGrid,
        }
    ));

}
}
fn move_creature(//creatures could have just moved comp?
mut creature_query: Query<(&mut Location, &mut RequestLocation, &mut DirectionFacing), With<Creature>>,
mut player_query: Query<&mut Location, With<Player>>,
mut obstacle_query: Query<&ObstacleLocation>,
mut move_event: EventReader<GlobalMoveEvent>,
    ) {
for _event in move_event.read() {
for (mut location, mut request_location, mut direction_facing) in &mut creature_query {

}
}
}
