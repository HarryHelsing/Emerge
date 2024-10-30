use bevy::prelude::*;


pub struct TilesPlugin;

#[derive(Event)]
pub struct UpdateTilesEvent;


impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateTilesEvent>();
        app.add_systems(Update, update_tiles);
    }
}

fn update_tiles(mut update_tiles_reader: EventReader<UpdateTilesEvent>) {
for _event in update_tiles_reader.read() {
println!("The event worked!");
}
}
