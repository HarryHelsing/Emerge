use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::turn_plugin::GlobalMoveEvent;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>();
        app.add_systems(Update, attack_damage);
        app.add_systems(Update, attack_image);
    }
}

#[derive(Event)]
pub struct AttackEvent {
    pub image_handle: Handle<Image>,
    pub damage: usize,
    pub grid_x: f32,
    pub grid_y: f32,
}//later could add attack type

fn attack_damage() {

}


fn attack_image() {

}
