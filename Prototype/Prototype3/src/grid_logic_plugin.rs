use bevy::prelude::*;
//use crate::tiles_plugin::UpdateTilesEvent;

pub struct GridLogicPlugin;

impl Plugin for GridLogicPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, snap_to_grid);
    }
}

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 9;
const CELL_SIZE: f32 = 128.0;
const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;

#[derive(PartialEq)]
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
pub grid_x: f32,
pub grid_y: f32,
}

#[derive(Component)]
pub struct ObstacleLocation {
pub obstacle: bool,
pub grid_x: f32,
pub grid_y: f32,
}

#[derive(Component)]
pub struct Offset {
pub offset: bool,
pub off_x: f32,
pub off_y: f32,
}

#[derive(Bundle)]
pub struct GridEntityBundle {
pub direction_facing: DirectionFacing,
pub location: Location,
pub request_location: RequestLocation,
pub obstacle_location: ObstacleLocation,
pub offset: Offset,
pub on_grid: OnGrid,
}

fn snap_to_grid(mut query: Query<(&Location, &mut Transform), With<OnGrid>>) {
    for (location, mut transform) in &mut query {
       let new_x = (location.grid_x * CELL_SIZE) - (SCREEN_WIDTH / 2.0) + 64.0;
       let new_y = (location.grid_y * CELL_SIZE) - (SCREEN_HEIGHT / 2.0) + 64.0;

        transform.translation.x = new_x;
        transform.translation.y = new_y;
    };
}

//snap to grid fn
//

