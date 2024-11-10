use bevy::prelude::*;
use crate::tiles_plugin::UpdateTilesEvent;

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
//    mut query: Query<(&GridPos, &mut UpdateGridPos, &mut InputDirection), With<Player>>,
//    obstacle_query: Query<&ObstaclePos>,
) {
    if keys.get_pressed().next().is_some() {
        //All this logic is based upon old movement logic
        //create a player direction component,
        //ability logic works off of which way you are facing
        //-first look for keyboard inputs,
        //change direction based upon this
        //-then within the same fn, check for button presses
        //-keep in mind tweaking for technical difficulties
        //such as multiple inputs
        let mut blocked = false;
        let mut new_x = 0.0;
        let mut new_y = 0.0;
    for (GridPos, mut UpdateGridPos, mut PlayerAttackDirection) in &mut query {
        UpdateGridPos.xgrid = GridPos.xgrid;
        UpdateGridPos.ygrid = GridPos.ygrid;

            PlayerAttackDirection.direction = Direction::None;
        if keys.pressed(KeyCode::KeyZ) {
            if keys.just_pressed(KeyCode::ArrowUp) {PlayerAttackDirection.direction = Direction::North}
            else if keys.just_pressed(KeyCode::ArrowDown) {PlayerAttackDirection.direction = Direction::South}
            else if keys.just_pressed(KeyCode::ArrowLeft) {PlayerAttackDirection.direction = Direction::West}
            else if keys.just_pressed(KeyCode::ArrowRight) {PlayerAttackDirection.direction = Direction::East}

        }//swap these code blocks,
         //also check for multiple inputs
         //check order:
         //direction and, move, or attack(swap leap&attack?), or leap, or summon?
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
                break;
        }
    }

