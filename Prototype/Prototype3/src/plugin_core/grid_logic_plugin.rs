use bevy::prelude::*;
use crate::{CELL_SIZE, SCREEN_WIDTH, SCREEN_HEIGHT};
//use crate::tiles_plugin::UpdateTilesEvent;

pub struct GridLogicPlugin;

impl Plugin for GridLogicPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, snap_to_grid);
        app.add_systems(Update, sync_obstacle_location);
    }
}

#[derive(PartialEq, Clone)]
pub enum Direction {
North, East, South, West,
}

#[derive(Component)]
pub struct OnGrid;

#[derive(Component)]
pub struct DirectionFacing {
pub facing: Direction,
}

#[derive(Component)]
pub struct Location {
pub grid_x: f32,
pub grid_y: f32,
}

#[derive(Component)]
pub struct RequestLocation {
pub requesting: bool,
pub can_move: bool,
pub grid_x: f32,
pub grid_y: f32,
}

#[derive(Component)]
pub struct ObstacleLocation {
pub is_obstacle: bool,
pub grid_x: f32,
pub grid_y: f32,
}

#[derive(Component)]
pub struct Offset {
pub offset: bool,
pub off_x: f32,
pub off_y: f32,
}

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerEntityBundle {
pub player: Player,
pub direction_facing: DirectionFacing,
pub location: Location,
pub obstacle_location: ObstacleLocation,
pub offset: Offset,
pub on_grid: OnGrid,
}


#[derive(Bundle)]
pub struct CreatureEntityBundle {
pub creature: Creature,
pub direction_facing: DirectionFacing,
pub location: Location,
pub request_location: RequestLocation,
pub obstacle_location: ObstacleLocation,
pub offset: Offset,
pub on_grid: OnGrid,
}

#[derive(Bundle)]
pub struct StaticEntityBundle {
pub location: Location,
pub obstacle_location: ObstacleLocation,
pub on_grid: OnGrid,
}

fn snap_to_grid(mut query: Query<(&Location, &mut Transform), With<OnGrid>>) {
    for (location, mut transform) in &mut query {
       let new_x = (location.grid_x * CELL_SIZE) - (SCREEN_WIDTH / 2.0) + 64.0;
       let new_y = (location.grid_y * CELL_SIZE) - (SCREEN_HEIGHT / 2.0) + 64.0;//make it cell size \ 2? less magic number
             //make it work for all screen res
        transform.translation.x = new_x;
        transform.translation.y = new_y;
    };
}

fn sync_obstacle_location(mut query: Query<(&Location, &mut ObstacleLocation), With<OnGrid>>) {
    for (location, mut obstacle_location) in &mut query {
        obstacle_location.grid_x = location.grid_x;
        obstacle_location.grid_y = location.grid_y;
    };
}//could i make this triggered by an event?
 //Also only check mobile things? unless I make rocks movable
