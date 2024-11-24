use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::turn_plugin::GlobalMoveEvent;
use crate::grid_logic_plugin::{CreatureEntityBundle, Direction, OnGrid, DirectionFacing, Location, RequestLocation, ObstacleLocation, Offset, Creature, Player};
use crate::health_plugin::Health;

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
            transform: Transform::from_xyz(0.0, 0.0, 7.0),
            ..Default::default()        
        },
        Health {
            max_hp: 20,
            current_hp: 20,
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
mut creature_query: Query<(&mut Location, &mut RequestLocation, &mut DirectionFacing), (With<Creature>, Without<Player>)>,
mut player_query: Query<&Location, (With<Player>, Without<Creature>)>,
mut obstacle_query: Query<&ObstacleLocation>,
mut move_event: EventReader<GlobalMoveEvent>,
    ) {
for _event in move_event.read() {
    let mut player_x = 0.0;
    let mut player_y = 0.0;
    let mut creature_x = 0.0;
    let mut creature_y = 0.0;
    let mut blocked = false;
        for location in &player_query {
    player_x = location.grid_x;
    player_y = location.grid_y;
        }

    for (mut location, mut request_location, mut direction_facing) in &mut creature_query {
        request_location.grid_x = location.grid_x;
        request_location.grid_y = location.grid_y;
        creature_x = location.grid_x;
        creature_y = location.grid_y;
        let difference_x = (player_x - creature_x).abs();
        let difference_y = (player_y - creature_y).abs();
        let where_x = creature_x - player_x;
        let where_y = creature_y - player_y;

        if difference_x >= difference_y {
                if where_x > 0.0 {request_location.grid_x = request_location.grid_x - 1.0}
                else {request_location.grid_x = request_location.grid_x + 1.0}
        } else {
                if where_y > 0.0 {request_location.grid_y = request_location.grid_y - 1.0}
                else {request_location.grid_y = request_location.grid_y + 1.0}
        }

        for obstacle_location in &obstacle_query {
     if  request_location.grid_x == obstacle_location.grid_x &&
         request_location.grid_y == obstacle_location.grid_y { blocked = true; } 
             }

     if blocked {request_location.grid_x = location.grid_x;
                 request_location.grid_y = location.grid_y;
                 blocked = false;
            if difference_x >= difference_y {
                if where_y > 0.0 {request_location.grid_y = request_location.grid_y - 1.0}
                else {request_location.grid_y = request_location.grid_y + 1.0}
          } else {
                if where_x > 0.0 {request_location.grid_x = request_location.grid_x - 1.0}
                else {request_location.grid_x = request_location.grid_x + 1.0}
         }
     }

        for obstacle_location in &obstacle_query {
     if  request_location.grid_x == obstacle_location.grid_x &&
         request_location.grid_y == obstacle_location.grid_y { blocked = true; } 
             }


     if !blocked {location.grid_x = request_location.grid_x;
                  location.grid_y = request_location.grid_y;}

}
}
}
