use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::turn_plugin::GlobalMoveEvent;
use crate::plugin_core::grid_logic_plugin::Direction;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAttackEvent>();
        app.add_event::<ChangePlayerAttackEvent>();
        app.insert_resource(PlayerAttackData {
            image_handle: Handle::default(),
            damage: 20,
            range: 1.0,
        });
        app.add_systems(Update, player_attack_damage);
        app.add_systems(Update, player_attack_image);
        app.add_systems(Update, creature_attack_damage);
        app.add_systems(Update, creature_attack_image);
    }
}

#[derive(Resource)]
pub struct PlayerAttackData {
    pub image_handle: Handle<Image>,
    pub damage: i32,
    pub range: f32,
}//later add width and enum for attack shape (eg, long, wide, square, round)

#[derive(Event)]
pub struct PlayerAttackEvent {
    pub grid_x: f32,
    pub grid_y: f32,
    pub direction: Direction,
}

#[derive(Event)]
pub struct ChangePlayerAttackEvent {
    pub image_handle: Handle<Image>,
    pub damage: i32,
    pub range: f32,
}

fn change_player_attack(
    mut change_attack_reader: EventReader<ChangePlayerAttackEvent>,
    mut attack_type: ResMut<PlayerAttackData>,
    ) {
    for event in change_attack_reader.read() {
        attack_type.image_handle = event.image_handle.clone();
        attack_type.damage = event.damage.clone();
        attack_type.range = event.range.clone();
    }
}


fn player_attack_damage(

    ) {
//look up player attack resource
}


fn player_attack_image(
    mut commands: Commands,
    mut attack_reader: EventReader<PlayerAttackEvent>,
    attack_type: Res<PlayerAttackData>,
    ) {
    for event in attack_reader {
        // read event and attack type to determine attack entity
    }
}

fn creature_attack_damage() {
//query creatures attacking
}


fn creature_attack_image() {

}
