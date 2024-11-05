use bevy::prelude::*;
use rand::Rng;

/*ToDo
 * Make animation event
 * Make animation struct with enum state machine
 * import plugin to main.rs
 * fn to update animation state (triggered by event?)
 * fn to update atlas index based upon state info then trigger event if needed
 * Q: what triggers the animation event from an idle state?
 * A: Global turn timer
 */

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<>();
        app.add_systems(Update, );
    }
}

