use bevy::prelude::*;
use crate::tiles_plugin::SetupEvent;
use crate::turn_plugin::GlobalMoveEvent;
use crate::grid_logic_plugin::OnGrid;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, check_alive);
    }
}

#[derive(Component)]
pub struct Health {
    pub max_hp: i32,
    pub current_hp: i32,
}

//health calculations

//query entitie's health with ongrid
fn check_alive(health_query: Query<(Entity, &Health), With<OnGrid>>,
               mut commands: Commands) {
    for (entity, health) in &health_query {
        if health.current_hp <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
