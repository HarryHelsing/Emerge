use bevy::prelude::*;
use crate::turn_plugin::GlobalMoveEvent;
use crate::turn_plugin::GlobalAnimateEvent;//merge into 1 crate
use rand::Rng;

pub struct AnimatePlugin;

#[derive(PartialEq)]
pub enum OpenCloseStates {
Open ,Closing, Closed, Opening,
}

struct FrameData {
first_frame: usize,
last_frame: usize,
}

struct AnimationType {
anim_type: usize,//two nested fields names animation type! fix it
states: [FrameData; 4],
}

#[derive(Resource)]
struct FourStateAnimation {
animation_type: Vec<AnimationType>,
}

#[derive(Component)]
pub struct AnimateOpenClose {
    pub animation_type: usize,
    pub no_movement: bool,
    pub reverse_animate: bool,//check last frame is smaller than first, set to true
    pub loop_animation: bool,
    pub animation_states:OpenCloseStates,//consider seperating this into
    pub animation_index: usize,          //its own component to keep it
    pub first_frame: usize,              //modular. Then creating a bundle?
    pub last_frame: usize,               //maybe include just_changed_state
    pub just_changed_state: bool,        //would work since not all need a state
}



impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animation_state_changer);
        app.add_systems(Update, animation_parser);
        app.insert_resource(FourStateAnimation {
            animation_type: vec![
                AnimationType {
                anim_type: 1,
                states: [
                    FrameData {first_frame: 15, last_frame: 15},
                    FrameData {first_frame: 14, last_frame: 13},
                    FrameData {first_frame: 12, last_frame: 12},
                    FrameData {first_frame: 11, last_frame: 10},
                ]},
                AnimationType {
                anim_type: 2,
                states: [
                    FrameData {first_frame: 20, last_frame: 20},
                    FrameData {first_frame: 19, last_frame: 17},
                    FrameData {first_frame: 16, last_frame: 16},
                    FrameData {first_frame: 17, last_frame: 19},
                ]},

            ],

        });
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
    OpenCloseStates::Open =>
    { AnimateOpenClose.animation_states = OpenCloseStates::Closing; AnimateOpenClose.just_changed_state = true}
    OpenCloseStates::Closing => {}
    OpenCloseStates::Closed =>
    { AnimateOpenClose.animation_states = OpenCloseStates::Opening; AnimateOpenClose.just_changed_state = true }
    OpenCloseStates::Opening => {}
            }
            }
}
}
}

fn animation_parser(
    mut global_animate_reader: EventReader<GlobalAnimateEvent>,
    mut query_animation: Query<(&mut TextureAtlas, &mut AnimateOpenClose)>,
    animation_data: Res<FourStateAnimation>,
    ) {
    
for _event in global_animate_reader.read() {
for (mut TextureAtlas, mut AnimateOpenClose) in query_animation.iter_mut() {
let mut keep_going = true;

//Update entity after state change
if AnimateOpenClose.just_changed_state {
keep_going = false;

let animation_vec = &animation_data.animation_type;
let target = AnimateOpenClose.animation_type;
for AnimationType in animation_vec.iter()
{
    if AnimationType.anim_type == target {
        match AnimateOpenClose.animation_states {
    OpenCloseStates::Open => {
        AnimateOpenClose.first_frame = AnimationType.states[0].first_frame;
        AnimateOpenClose.last_frame = AnimationType.states[0].last_frame;
    }
    OpenCloseStates::Closing => {
        AnimateOpenClose.first_frame = AnimationType.states[1].first_frame;
        AnimateOpenClose.last_frame = AnimationType.states[1].last_frame;
    }
    OpenCloseStates::Closed => {
        AnimateOpenClose.first_frame = AnimationType.states[2].first_frame;
        AnimateOpenClose.last_frame = AnimationType.states[2].last_frame;
    }
    OpenCloseStates::Opening => {
        AnimateOpenClose.first_frame = AnimationType.states[3].first_frame;
        AnimateOpenClose.last_frame = AnimationType.states[3].last_frame;
    }
        }
TextureAtlas.index = AnimateOpenClose.first_frame;
    }

}
if AnimateOpenClose.first_frame == AnimateOpenClose.last_frame {
    AnimateOpenClose.no_movement = true } else {
    AnimateOpenClose.no_movement = false }

if AnimateOpenClose.first_frame > AnimateOpenClose.last_frame {
    AnimateOpenClose.reverse_animate = true } else {
    AnimateOpenClose.reverse_animate = false }

AnimateOpenClose.just_changed_state = false;

//then apply logic based off of frames to define animateOC fields
    }

//Iterate through animation
if AnimateOpenClose.no_movement { keep_going = false }
if keep_going {
if AnimateOpenClose.reverse_animate {
TextureAtlas.index = TextureAtlas.index -1;
        } else {
TextureAtlas.index = TextureAtlas.index +1;
        }

if TextureAtlas.index == AnimateOpenClose.last_frame {
match AnimateOpenClose.animation_states {
    OpenCloseStates::Open => {}
    OpenCloseStates::Closing =>
{ AnimateOpenClose.animation_states = OpenCloseStates::Closed; AnimateOpenClose.just_changed_state = true}
    OpenCloseStates::Closed => {}
    OpenCloseStates::Opening =>
    { AnimateOpenClose.animation_states = OpenCloseStates::Open; AnimateOpenClose.just_changed_state = true }

            }
        }
    }
        }
    }
}
//Create iterative animator that sends animation event to change state when at the end of cycle
