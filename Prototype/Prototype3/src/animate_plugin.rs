use bevy::prelude::*;
use crate::turn_plugin::{GlobalSecondEvent, GlobalAnimateEvent};
use rand::Rng;

pub struct AnimatePlugin;

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

#[derive(PartialEq)]//change to FourStates?
pub enum OpenCloseStates {//more modular
Open ,Closing, Closed, Opening,
}

struct FrameData {
first_frame: usize,
last_frame: usize,
}

struct AnimationType {
anim_type: usize,
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
    pub reverse_animate: bool,
    pub loop_animation: bool,
    pub animation_states:OpenCloseStates,
    pub first_frame: usize,
    pub last_frame: usize,
    pub just_changed_state: bool,
}
//consider seperating this into
//its own component to keep it
//modular. Then creating a bundle?
//maybe include just_changed_state
//would work since not all need a state

fn animation_state_changer(
    mut global_second_reader: EventReader<GlobalSecondEvent>,
    mut decor_query: Query<&mut AnimateOpenClose>,
    ) {
for _event in global_second_reader.read() {
for mut animate_open_close in decor_query.iter_mut() {
    let mut rng = rand::thread_rng();
            let decor_move = rng.gen_bool(0.1); 
            if decor_move {
            match animate_open_close.animation_states {
    OpenCloseStates::Open =>
    { animate_open_close.animation_states = OpenCloseStates::Closing; animate_open_close.just_changed_state = true}
    OpenCloseStates::Closing => {}
    OpenCloseStates::Closed =>
    { animate_open_close.animation_states = OpenCloseStates::Opening; animate_open_close.just_changed_state = true }
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
for (mut texture_atlas, mut animate_open_close) in query_animation.iter_mut() {
let mut keep_going = true;

//Update entity after state change
if animate_open_close.just_changed_state {
keep_going = false;

let animation_vec = &animation_data.animation_type;
let target = animate_open_close.animation_type;
for animation_type in animation_vec.iter()
{
    if animation_type.anim_type == target {
        match animate_open_close.animation_states {
    OpenCloseStates::Open => {
        animate_open_close.first_frame = animation_type.states[0].first_frame;
        animate_open_close.last_frame = animation_type.states[0].last_frame;
    }
    OpenCloseStates::Closing => {
        animate_open_close.first_frame = animation_type.states[1].first_frame;
        animate_open_close.last_frame = animation_type.states[1].last_frame;
    }
    OpenCloseStates::Closed => {
        animate_open_close.first_frame = animation_type.states[2].first_frame;
        animate_open_close.last_frame = animation_type.states[2].last_frame;
    }
    OpenCloseStates::Opening => {
        animate_open_close.first_frame = animation_type.states[3].first_frame;
        animate_open_close.last_frame = animation_type.states[3].last_frame;
    }
        }
texture_atlas.index = animate_open_close.first_frame;
    }

}
if animate_open_close.first_frame == animate_open_close.last_frame {
    animate_open_close.no_movement = true } else {
    animate_open_close.no_movement = false }

if animate_open_close.first_frame > animate_open_close.last_frame {
    animate_open_close.reverse_animate = true } else {
    animate_open_close.reverse_animate = false }

animate_open_close.just_changed_state = false;

//then apply logic based off of frames to define animateOC fields
    }

//Iterate through animation
if animate_open_close.no_movement { keep_going = false }
if keep_going {
if animate_open_close.reverse_animate {
texture_atlas.index = texture_atlas.index -1;
        } else {
texture_atlas.index = texture_atlas.index +1;
        }

if texture_atlas.index == animate_open_close.last_frame {
match animate_open_close.animation_states {
    OpenCloseStates::Open => {}
    OpenCloseStates::Closing =>
{ animate_open_close.animation_states = OpenCloseStates::Closed; animate_open_close.just_changed_state = true}
    OpenCloseStates::Closed => {}
    OpenCloseStates::Opening =>
    { animate_open_close.animation_states = OpenCloseStates::Open; animate_open_close.just_changed_state = true }

            }
        }
    }
        }
    }
}
//Create iterative animator that sends animation event to change state when at the end of cycle
