use bevy::prelude::*;
use crate::{GRID_HEIGHT, GRID_WIDTH};
use crate::grid_logic_plugin::{Direction, DirectionFacing, Location, ObstacleLocation, Player};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, keyboard_input);
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Location, &mut DirectionFacing), With<Player>>,
    obstacle_query: Query<&ObstacleLocation>,
) {
    if keys.get_pressed().next().is_some() {
        //ability logic works off of which way you are facing
        //-keep in mind tweaking for technical difficulties
        //such as multiple inputs
    for (mut location, mut direction_facing) in &mut query {

            if keys.just_pressed(KeyCode::KeyW) {direction_facing.facing = Direction::North}
            else if keys.just_pressed(KeyCode::KeyS) {direction_facing.facing = Direction::South}
            else if keys.just_pressed(KeyCode::KeyA) {direction_facing.facing = Direction::West}
            else if keys.just_pressed(KeyCode::KeyD) {direction_facing.facing = Direction::East}


        let mut new_x = location.grid_x;
        let mut new_y = location.grid_y;
        let mut blocked = false;
    if keys.just_pressed(KeyCode::KeyJ) {println!("Move");
    if direction_facing.facing == Direction::North {new_y = new_y + 1.0}
    else if direction_facing.facing == Direction::South {new_y = new_y - 1.0}
    else if direction_facing.facing == Direction::East {new_x = new_x + 1.0}
    else if direction_facing.facing == Direction::West {new_x = new_x - 1.0}
    }
        else if keys.just_pressed(KeyCode::KeyK) {println!("Attack")}
        else if keys.just_pressed(KeyCode::KeyL) {println!("Leap");
    if direction_facing.facing == Direction::North {new_y = new_y + 2.0}
    else if direction_facing.facing == Direction::South {new_y = new_y - 2.0}
    else if direction_facing.facing == Direction::East {new_x = new_x + 2.0}
    else if direction_facing.facing == Direction::West {new_x = new_x - 2.0}
        }
        else if keys.just_pressed(KeyCode::Semicolon) {println!("Summon")}
            //logic for J: move, K: attack, L: leap, ;: summon
            //even if just using println! to show it's working
    for obstacle_location in &obstacle_query {
        if obstacle_location.is_obstacle {
        if  new_x == obstacle_location.grid_x && new_y == obstacle_location.grid_y {
            blocked = true;
            break;}
        }
            }
    if blocked {break;}
        if new_x >= 0.0 && new_x < GRID_WIDTH as f32
            && new_y >= 0.0 && new_y < GRID_HEIGHT as f32 {
           location.grid_x = new_x;
           location.grid_y = new_y;
            }
        }
    }
}
