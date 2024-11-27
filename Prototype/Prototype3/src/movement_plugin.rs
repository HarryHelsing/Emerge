use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::turn_plugin::GlobalMoveEvent;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<AttackEvent>();
        app.add_systems(Update, );
    }
}

/* add movement fn that takes in event from input,
 * then sends valid/invalid movement data in event
 * that event is read by a system to handle movement
 * and one to designed to send event to the global move fn
 * in the turn plugin
