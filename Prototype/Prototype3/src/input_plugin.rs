use bevy::prelude::*;
use crate::tiles_plugin::UpdateTilesEvent;
use crate::grid_logic_plugin::{Direction, OnGrid, DirectionFacing, Location, ObstacleLocation};
use crate::player_plugin::Player;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, keyboard_input);
    }
}

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 9;
const CELL_SIZE: f32 = 128.0;

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Location, &mut DirectionFacing), With<Player>>,
    obstacle_query: Query<&ObstacleLocation>,
) {
    println!("Bug test 1");
    if keys.get_pressed().next().is_some() {
    println!("Bug test 2");
        //All this logic is based upon old movement logic
        //ability logic works off of which way you are facing
        //-first look for keyboard inputs,
        //change direction based upon this
        //-then within the same fn, check for button presses
        //-keep in mind tweaking for technical difficulties
        //such as multiple inputs
    for (mut location, mut direction_facing) in &mut query {

    println!("Bug test 3");
            if keys.just_pressed(KeyCode::KeyW) {direction_facing.facing = Direction::North}
            else if keys.just_pressed(KeyCode::KeyS) {direction_facing.facing = Direction::South}
            else if keys.just_pressed(KeyCode::KeyA) {direction_facing.facing = Direction::West}
            else if keys.just_pressed(KeyCode::KeyD) {direction_facing.facing = Direction::East}


    if keys.just_pressed(KeyCode::KeyJ) {println!("Move")}//move in facing direction, use logic.
        else if keys.just_pressed(KeyCode::KeyK) {println!("Attack")}
        else if keys.just_pressed(KeyCode::KeyL) {println!("Leap")}
        else if keys.just_pressed(KeyCode::Semicolon) {println!("Summon")}
            //logic for J: move, K: attack, L: leap, ;: summon
            //even if just using println! to show it's working
        }
    }
}
/*     
        else
            if keys.just_pressed(KeyCode::ArrowUp) {
               if UpdateGridPos.ygrid < F_GRID_HEIGHT - 1.0 {UpdateGridPos.ygrid += 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowDown) {
               if UpdateGridPos.ygrid > 0.0 {UpdateGridPos.ygrid -= 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowLeft) {
               if UpdateGridPos.xgrid > 0.0 {UpdateGridPos.xgrid -= 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowRight) {
               if UpdateGridPos.xgrid < F_GRID_WIDTH - 1.0 {UpdateGridPos.xgrid += 1.0;}
            }
        new_x = UpdateGridPos.xgrid;
        new_y = UpdateGridPos.ygrid;
            for ObstaclePos in &obstacle_query {
            if  new_x == ObstaclePos.xgrid && new_y ==  ObstaclePos.ygrid {
                blocked = true;
        let mut blocked = false;
        let mut new_x = 0.0;
        let mut new_y = 0.0;

        if keys.pressed(KeyCode::KeyZ) {
                break;*/
