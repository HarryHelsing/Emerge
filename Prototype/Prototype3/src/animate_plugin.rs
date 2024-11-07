use bevy::prelude::*;
use crate::turn_plugin::GlobalMoveEvent;
use rand::Rng;

/*ToDo
 * Make animation event
 * fn to update atlas index based upon state info then trigger event if needed
 * add a resource with a hash map to store structs containing relevant data
 */

pub struct AnimatePlugin;

//define animation state enum
//make component for all animation info
#[derive(PartialEq)]
pub enum OpenCloseStates {
Open ,Closing, Closed, Opening,
}

#[derive(Component)]
pub struct AnimateOpenClose {
    pub animation_type: usize,
    pub animate: bool,
    pub reverse_animate: bool,//check last frame is smaller than first, set to true
    pub loop_animation: bool,
    pub animation_states:OpenCloseStates,//consider seperating this into
    pub animation_index: usize,          //its own component to keep it
    pub first_frame: usize,              //modular. Then creating a bundle?
    pub last_frame: usize,               //maybe include just_changed_state
    pub just_changed_state: bool,
}



impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, animation_state_changer);
    }
}

//fn for updating animation frame
//fn for updating state and relevant data on animation component
//(I sense magic numbers coming)-we will fix this with resources
fn animation_state_changer(
    mut global_move_reader: EventReader<GlobalMoveEvent>,
    mut decor_query: Query<&mut AnimateOpenClose>,
    ) {
for _event in global_move_reader.read() {
println!("Global time is working inside the animate plugin, hell yeah");
for mut AnimateOpenClose in decor_query.iter_mut() {
    let mut rng = rand::thread_rng();
            let decor_move = rng.gen_bool(0.5); 
            if decor_move {
            match AnimateOpenClose.animation_states {
    OpenCloseStates::Open => { AnimateOpenClose.animation_states = OpenCloseStates::Closing; AnimateOpenClose.just_changed_state = true}
    OpenCloseStates::Closing => {}
    OpenCloseStates::Closed => { AnimateOpenClose.animation_states = OpenCloseStates::Opening; AnimateOpenClose.just_changed_state = true }
    OpenCloseStates::Opening => {}
            }
            }
}
}
}

fn animation_parser(
//query animation components, resource stuff for changing sprites,
//resource for animation frame details
    ) {

}
//Create iterative animator that sends animation event to change state when at the end of cycle
