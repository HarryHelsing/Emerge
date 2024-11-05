use bevy::prelude::*;
use rand::Rng;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<>();
        app.add_systems(Update, );
    }
}

fn global_time() {
//Count time, interrupted by action, sends event at completion for world move
}
