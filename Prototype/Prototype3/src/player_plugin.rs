use bevy::prelude::*;
use crate::tiles_plugin::UpdateTilesEvent;

pub struct PlayerPlugin;

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 9;
const CELL_SIZE: f32 = 128.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, spawn_player);
    }
}

fn spawn_player(
mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Assets<Image>>, 
    mut update_tiles_reader: EventReader<UpdateTilesEvent>,
    ) {
for _event in update_tiles_reader.read() {
    let texture_handle_frog = asset_server.load("player/drowsy_frog_sprite.png");                                                                                     

    commands.spawn((
        SpriteBundle {
            texture: texture_handle_frog.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()        
        },
    ));

}
}
