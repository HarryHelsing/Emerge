use bevy::prelude::*;

const CELL_SIZE: f32 = 10.0; // Each cell is 8x8 pixels


// Function to spawn a grass tile
pub fn spawn_tile(commands: &mut Commands, texture_handle: Handle<Image>, x: usize, y: usize) {
    let position = Vec3::new(
        x as f32 * CELL_SIZE - 155.0,
        y as f32 * CELL_SIZE - 85.0,
        0.0,
    );

    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform {
            translation: position,
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
pub fn spawn_blob(commands: &mut Commands, texture_handle: Handle<Image>, x: usize, y: usize) {
    let position = Vec3::new(
        x as f32 * CELL_SIZE - 155.0,
        y as f32 * CELL_SIZE - 85.0,
        1.0,
    );

    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform {
            translation: position,
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

