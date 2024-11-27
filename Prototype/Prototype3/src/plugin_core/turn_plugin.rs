use bevy::prelude::*;
use crate::input_plugin::ActionTakenEvent;

pub struct TurnPlugin;
//rename to time plugin?
#[derive(Event)]
pub struct GlobalMoveEvent;

#[derive(Event)]
pub struct GlobalSecondEvent;

#[derive(Event)]
pub struct GlobalAnimateEvent;

#[derive(Resource)]
struct FiveSecondTimer(Timer);

#[derive(Resource)]
struct OneSecondTimer(Timer);

#[derive(Resource)]
struct ZeroPointTwoSecondTimer(Timer);

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
    app.insert_resource(FiveSecondTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
    app.insert_resource(OneSecondTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
    app.insert_resource(ZeroPointTwoSecondTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
        app.add_event::<GlobalMoveEvent>();
        app.add_event::<GlobalSecondEvent>();
        app.add_event::<GlobalAnimateEvent>();
        app.add_systems(Update, global_time);
        app.add_systems(Update, global_second);
        app.add_systems(Update, global_animate);
    }
}

//Create interupt feature by receiving event from player input fn
fn global_time(
time: Res<Time>,
mut timer: ResMut<FiveSecondTimer>,
mut global_move_writer: EventWriter<GlobalMoveEvent>,
mut action_taken_reader: EventReader<ActionTakenEvent>,
    ) { 
for _event in action_taken_reader.read() {
   //reset timer! 
    timer.0.reset();
    global_move_writer.send(GlobalMoveEvent);
}
if timer. 0.tick(time.delta()).just_finished() {
    global_move_writer.send(GlobalMoveEvent);
}
}

fn global_second(
time: Res<Time>,
mut timer: ResMut<OneSecondTimer>,
mut global_move_writer: EventWriter<GlobalSecondEvent>,
    ) { 
if timer. 0.tick(time.delta()).just_finished() {
    global_move_writer.send(GlobalSecondEvent);
}
}


fn global_animate(
time: Res<Time>,
mut timer: ResMut<ZeroPointTwoSecondTimer>,
mut global_animate_writer: EventWriter<GlobalAnimateEvent>,
    ) { 
if timer. 0.tick(time.delta()).just_finished() {
    global_animate_writer.send(GlobalAnimateEvent);
}
}


//Count time, interrupted by action, sends event at completion for world move
//Future consideration, how to handle interrupted timer when player moves?
