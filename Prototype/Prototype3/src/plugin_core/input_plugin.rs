use bevy::prelude::*;
use crate::{GRID_HEIGHT, GRID_WIDTH};
use crate::grid_logic_plugin::{Direction, DirectionFacing, Location, ObstacleLocation, Player};
use crate::plugin_combat::attack_plugin::AttackEvent;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionTakenEvent>();
        app.add_systems(Update, keyboard_input);
    }
}

#[derive(Event)]
pub struct ActionTakenEvent;

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Location, &mut DirectionFacing), With<Player>>,
    obstacle_query: Query<&ObstacleLocation>,
    mut writers: EventWriter<ActionTakenEvent>,
    //refactor, send key pressed event to different fn.
    //I have created a monolith function
    //-break into movement fn, leap, attack. with direction & location
    //check if inputs are valid in other fns, then send actiontaken event
//    mut writers: (EventWriter<ActionTakenEvent>, EventWriter<AttackEvent>),
//    mut attack_writer: EventWriter<AttackEvent>,
) {
    if keys.get_pressed().next().is_some() {
        //ability logic works off of which way you are facing
    for (mut location, mut direction_facing) in &mut query {

            if keys.just_pressed(KeyCode::KeyW) {direction_facing.facing = Direction::North}
            else if keys.just_pressed(KeyCode::KeyS) {direction_facing.facing = Direction::South}
            else if keys.just_pressed(KeyCode::KeyA) {direction_facing.facing = Direction::West}
            else if keys.just_pressed(KeyCode::KeyD) {direction_facing.facing = Direction::East}


        let mut new_x = location.grid_x;
        let mut new_y = location.grid_y;
        let mut blocked = false;
    if keys.just_pressed(KeyCode::KeyJ) {println!("Move");//Send move event
    writers.send(ActionTakenEvent);
    //Unfortunately the way I've structured the code making
    //action writer conditional upon a *succesful* move is tricky
    match direction_facing.facing {
        Direction::North => {new_y = new_y + 1.0}
        Direction::South => {new_y = new_y - 1.0}
        Direction::East => {new_x = new_x + 1.0}
        Direction::West => {new_x = new_x - 1.0}
    }
}
        else if keys.just_pressed(KeyCode::KeyK) {println!("Attack");
    writers.send(ActionTakenEvent);
    //if statements? Or match statement for direction,
    //player location plus direction for attack location
        }//Send move event
        else if keys.just_pressed(KeyCode::KeyL) {println!("Leap");//Send move event
   writers.send(ActionTakenEvent);
     match direction_facing.facing {
        Direction::North => {new_y = new_y + 2.0}
        Direction::South => {new_y = new_y - 2.0}
        Direction::East => {new_x = new_x + 2.0}
        Direction::West => {new_x = new_x - 2.0}
    }
}
        else if keys.just_pressed(KeyCode::KeyI) {println!("Summon");
    writers.send(ActionTakenEvent);
        }//Send move event
            //logic for J: move, K: attack, L: leap, ;: summon
            //even if just using println! to show it's working
    for obstacle_location in &obstacle_query {
        if obstacle_location.is_obstacle {
        if  new_x == obstacle_location.grid_x && new_y == obstacle_location.grid_y {
            blocked = true;
            break;}
        }
            }
    if blocked {break;}//What are these breaks for?
        if new_x >= 0.0 && new_x < GRID_WIDTH as f32
            && new_y >= 0.0 && new_y < GRID_HEIGHT as f32 {
           location.grid_x = new_x;
           location.grid_y = new_y;
            }
        }
    }
}
