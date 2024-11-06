use bevy::prelude::*;
use crate::turn_plugin::GlobalMoveEvent;
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

//define animation state enum
//make component for all animation info



impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, test_time_reader2);
    }
}

//fn for updating animation frame
//fn for updating state and relevant data on animation component
//(I sense magic numbers coming)
fn test_time_reader2(
    mut update_tiles_reader: EventReader<GlobalMoveEvent>,
    ) {
for _event in update_tiles_reader.read() {
println!("Global time is working inside the animate plugin, hell yeah");
}
}
