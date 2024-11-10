use bevy::prelude::*;
use crate::animate_plugin::AnimateOpenClose;
use crate::animate_plugin::OpenCloseStates;

use rand::Rng;

pub struct TilesPlugin;

#[derive(Component)]
struct IsTile;

#[derive(Event)]
pub struct UpdateTilesEvent;

const GRID_WIDTH: usize = 15;
const GRID_HEIGHT: usize = 9;//This stuff will be outdated when fluid movement/world chunks is introduced
const CELL_SIZE: f32 = 128.0;//-side note: with chunks, should decoration be a child of chunk? look into it
/*Code is getting a tad messy. Could do with being more modular.
 *Too many magic numbers and hard coded things.
 *learn ron.
 */
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateTilesEvent>();
        app.add_systems(Update, update_tiles);
        app.add_systems(Update, update_decorations);
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



fn update_decorations(
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
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 5, 5, None, None);
    let atlas_image: Handle<Image> = asset_server.load("tile_decoration/dune_decoration.png");
    let atlas_layout = texture_atlas_layouts.add(layout);
    let mut rng = rand::thread_rng();
    let mut grid: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [
        [0; GRID_WIDTH]; GRID_HEIGHT];

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {            
            grid[y][x] = rng.gen_range(1..=30); }} 

    // Spawn the sprites based on the grid
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {          
          match grid[y][x] {   
                1 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 0),       
                2 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 1),       
                3 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 2),       
                4 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 3),       
                5 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 4),       
                6 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 5),       
                7 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 6),       
                8 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 7),       
                9 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 8),       
                10 => spawn_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 9),       
                11 => spawn_animated_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 12, 1, true, true, false, OpenCloseStates::Closed, 12, 12, false),       
                12 => spawn_animated_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 15, 1, true, true, false, OpenCloseStates::Open, 15, 15, false),       
                13 => spawn_animated_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 16, 2, true, false, false, OpenCloseStates::Closed, 16, 16, false),       
                14 => spawn_animated_decoration(&mut commands, atlas_image.clone(), atlas_layout.clone(), x, y, 20, 2, true, true, false, OpenCloseStates::Open, 20, 20, false),       
                _ => {}
            }
        }
    }

}
}

//Add fn or option for animated thing, creates animated component, remember to import crate from
//animate_plugin
fn spawn_decoration(commands: &mut Commands, image_handle: Handle<Image>, layout_handle: Handle<TextureAtlasLayout>, x: usize, y: usize, index: usize) {

    let mut rng = rand::thread_rng();
            let location_top = rng.gen_bool(0.5); 
            let location_right = rng.gen_bool(0.5); 
            let mut off_centre_x = -32.0;
            let mut off_centre_y = -32.0;
            if location_top {off_centre_y = 32.0};
            if location_right {off_centre_x = 32.0};
            let mut depth = 10.0;
            if location_top {depth = 5.0};
    let position = Vec3::new(
        x as f32 * CELL_SIZE - 896.0 + off_centre_x,
        y as f32 * CELL_SIZE - 540.0 + off_centre_y,
        depth,
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

fn spawn_animated_decoration(
    commands: &mut Commands,
    image_handle: Handle<Image>,
    layout_handle: Handle<TextureAtlasLayout>,
    x: usize,
    y: usize,
    index: usize,
    animation_type: usize,
    no_movement: bool,
    reverse_animate: bool,
    loop_animation: bool,
    animation_states:OpenCloseStates,
    first_frame: usize,
    last_frame: usize,
    just_changed_state: bool,
    ) {

    let mut rng = rand::thread_rng();
            let location_top = rng.gen_bool(0.5); 
            let location_right = rng.gen_bool(0.5); 
            let mut off_centre_x = -32.0;
            let mut off_centre_y = -32.0;
            if location_top {off_centre_y = 32.0};
            if location_right {off_centre_x = 32.0};
            let mut depth = 10.0;
            if location_top {depth = 5.0};
    let position = Vec3::new(
        x as f32 * CELL_SIZE - 896.0 + off_centre_x,
        y as f32 * CELL_SIZE - 540.0 + off_centre_y,
        depth,
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
        AnimateOpenClose {
    animation_type: animation_type,
    no_movement: no_movement,
    reverse_animate: reverse_animate,
    loop_animation: loop_animation,
    animation_states:animation_states,
    animation_index: index,
    first_frame: first_frame,
    last_frame: last_frame,
    just_changed_state: just_changed_state,
        },
    ));
}
