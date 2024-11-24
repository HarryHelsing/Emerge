use bevy::prelude::*;
use rand::Rng;
use crate::{CELL_SIZE, SCREEN_WIDTH, SCREEN_HEIGHT, GRID_HEIGHT, GRID_WIDTH};
use crate::tiles_plugin::SetupEvent;
use crate::grid_logic_plugin::{StaticEntityBundle, OnGrid, Location, ObstacleLocation,};
use crate::health_plugin::Health;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
//        app.add_event::<>();
        app.add_systems(Update, create_obstacles);
    }
}

fn create_obstacles(
    mut setup_reader: EventReader<SetupEvent>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts1: ResMut<Assets<TextureAtlasLayout>>,
    ) {
for _event in setup_reader.read() {

    let layout1 = TextureAtlasLayout::from_grid(UVec2::splat(128), 3, 2, None, None);
    let atlas_image1: Handle<Image> = asset_server.load("objects/rock_atlas.png");
    let atlas_layout1 = texture_atlas_layouts1.add(layout1);
    let mut rng = rand::thread_rng();
    let mut grid: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [
        [0; GRID_WIDTH]; GRID_HEIGHT];

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {            
            grid[y][x] = rng.gen_range(1..=26); }}                                          

    // Spawn the sprites based on the grid
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {          
          match grid[y][x] {   
                1 => spawn_object(&mut commands, atlas_image1.clone(), atlas_layout1.clone(), x, y, 0),       
                2 => spawn_object(&mut commands, atlas_image1.clone(), atlas_layout1.clone(), x, y, 1),       
                3 => spawn_object(&mut commands, atlas_image1.clone(), atlas_layout1.clone(), x, y, 2),       
                4 => spawn_object(&mut commands, atlas_image1.clone(), atlas_layout1.clone(), x, y, 3),       
                5 => spawn_object(&mut commands, atlas_image1.clone(), atlas_layout1.clone(), x, y, 4),       
                6 => spawn_object(&mut commands, atlas_image1.clone(), atlas_layout1.clone(), x, y, 5),       
                _ => {}
            }
        }
    }
}
}

fn spawn_object(commands: &mut Commands, image_handle1: Handle<Image>, layout_handle1: Handle<TextureAtlasLayout>, x: usize, y: usize, index: usize) {
    let position = Vec3::new(
    x as f32 * CELL_SIZE - (SCREEN_WIDTH / 2.0) + 64.0,
    y as f32 * CELL_SIZE - (SCREEN_HEIGHT / 2.0) + 64.0,
        8.0,
    );
let new_x = x as f32;
let new_y = y as f32;
    commands.spawn((
            SpriteBundle {
        texture: image_handle1,
        transform: Transform {
            translation: position,
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        ..Default::default()
    },
        TextureAtlas {
           layout: layout_handle1,
           index: index,
        },
        Health {
            max_hp: 40,
            current_hp: 40,
},
    StaticEntityBundle {
    location: Location { grid_x: new_x, grid_y: new_y },
    obstacle_location: ObstacleLocation { is_obstacle: true, grid_x: new_x, grid_y: new_y },
    on_grid: OnGrid,
        }
    ));
}
