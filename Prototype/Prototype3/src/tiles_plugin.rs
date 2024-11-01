use bevy::prelude::*;

use rand::Rng;

pub struct TilesPlugin;

#[derive(Component)]
struct IsTile;

#[derive(Event)]
pub struct UpdateTilesEvent;

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 10;
const CELL_SIZE: f32 = 128.0;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateTilesEvent>();
        app.add_systems(Update, update_tiles);
    }
}

fn update_tiles(
    query: Query<Entity, With<IsTile>>,
    mut update_tiles_reader: EventReader<UpdateTilesEvent>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {

for _event in update_tiles_reader.read() {
println!("The event worked!");

for entity in query.iter() {
commands.entity(entity).despawn();
}
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 4, 4, None, None);
    let atlas_image: Handle<Image> = asset_server.load("tiles/dune_atlas.png");
    let atlas_layout = texture_atlas_layouts.add(layout);
    let mut rng = rand::thread_rng();
    let mut grid: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [
        [0; GRID_WIDTH]; GRID_HEIGHT];

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {            
            grid[y][x] = rng.gen_range(1..=6); }}                                          

    // Spawn the sprites based on the grid
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {          
          match grid[y][x] {   
                1 => spawn_tile(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 0),       
                2 => spawn_tile(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 1),       
                3 => spawn_tile(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 2),       
                4 => spawn_tile(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 3),       
                5 => spawn_tile(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 4),       
                6 => spawn_tile(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 5),       
                _ => {}
            }
        }
    }

}
}

fn spawn_tile(commands: &mut Commands, image_handle: Handle<Image>, layout_handle: Handle<TextureAtlasLayout>, x: usize, y: usize, index: usize) {
    let position = Vec3::new(
        x as f32 * CELL_SIZE - 896.0,
        y as f32 * CELL_SIZE - 540.0,
        0.0,
    );

    commands.spawn((
            SpriteBundle {
        texture: image_handle,
        transform: Transform {
            translation: position,
            scale: Vec3::splat(1.0),
            ..Default::default()
        },
        ..Default::default()
    },
        TextureAtlas {
            layout: layout_handle,
           index: index,
        },
        IsTile,
    ));
}



