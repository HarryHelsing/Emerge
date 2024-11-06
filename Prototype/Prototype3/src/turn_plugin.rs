use bevy::prelude::*;

pub struct TurnPlugin;

#[derive(Event)]
pub struct GlobalMoveEvent;

#[derive(Resource)]
struct FiveSecondTimer(Timer);

//create 4s repeating timer resource
impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
    app.insert_resource(FiveSecondTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
        app.add_event::<GlobalMoveEvent>();
        app.add_systems(Update, global_time);
        app.add_systems(Update, test_time_reader);
    }
}

fn global_time(
time: Res<Time>,
mut timer: ResMut<FiveSecondTimer>,
mut global_move_writer: EventWriter<GlobalMoveEvent>,
    ) { 
if timer. 0.tick(time.delta()).just_finished() {
    global_move_writer.send(GlobalMoveEvent);
}
}

fn test_time_reader(
    mut update_tiles_reader: EventReader<GlobalMoveEvent>,
    ) {
for _event in update_tiles_reader.read() {
println!("Global time is working, hell yeah");
}
}

//Count time, interrupted by action, sends event at completion for world move
//Future consideration, how to handle interrupted timer when player moves?
