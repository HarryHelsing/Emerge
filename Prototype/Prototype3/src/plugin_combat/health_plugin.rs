use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::turn_plugin::GlobalMoveEvent;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, check_alive);
    }
}

#[derive(Component)]
pub struct Health {
    pub max_hp: i32,
    pub current_hp: i32,
}

//health calculations

fn check_alive() {

}
