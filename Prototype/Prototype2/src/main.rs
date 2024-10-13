use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResized;
use tiles::*;
use rand::Rng;
mod tiles;

const GRID_WIDTH: usize = 32;
const GRID_HEIGHT: usize = 18;
const F_GRID_WIDTH: f32 = GRID_WIDTH as f32;
const F_GRID_HEIGHT: f32 = GRID_HEIGHT as f32;
const SCREEN_WIDTH: f32 = 320.0;
const SCREEN_HEIGHT: f32 = 180.0;
const CELL_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (320.0, 180.0).into(),
                    resizable: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .insert_resource(FiveSecondTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .insert_resource(TwoSecondTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(OneSecondTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)//Optimisations
        .add_systems(Update, resize_camera)//done
        .add_systems(Update, move_entities)//After world tick
        .add_systems(Update, update_pos.after(move_entities))
        .add_systems(Update, keyboard_movement)//done
        .add_systems(Update, remove_attack_image)//After what? timer?
        .add_systems(Update, health_check)//After damage event? Or health change
        .add_systems(Update, remove_dead)
        .add_systems(Update, create_creature_attack.after(update_pos))
        .add_systems(Update, create_player_attack.after(move_entities))
        .add_systems(Update, apply_damage.before(remove_attack_damage))
        .add_systems(Update, remove_attack_damage.after(apply_damage))
        .add_systems(Update, timed_spawner)
        .add_systems(Update, timed_movement.before(timed_attack))
        .add_systems(Update, timed_attack.before(create_player_attack))
        .run();//key events? health_change, tick, attack
}
#[derive(Component)]
struct Player;

#[derive(Component)]
struct OnMap;

#[derive(Component)]
struct Creature;

#[derive(Component)]
struct GridPos {
    xgrid: f32,
    ygrid: f32,
}

#[derive(Component)]
struct UpdateGridPos {
    xgrid: f32,
    ygrid: f32,
}

#[derive(Component)]
struct ObstaclePos {
    xgrid: f32,
    ygrid: f32,
}

#[derive(Component)]
struct Health {
    health: i32,
    max_health: i32,
}

#[derive(Component)]
struct Alive {
    remove: bool,
}

#[derive(Component)]
struct JustMoved {
    just_moved: bool,
}

#[derive(Component)]
struct AttackImage {
    time_visual: Timer,
    remove: bool,
    xgrid: f32,
    ygrid: f32,
}

#[derive(Component)]
struct AttackDamage {
    attack_damage: i32,
    remove: bool,
    xgrid: f32,
    ygrid: f32,
}

#[derive(PartialEq)]
enum Direction {
North, East, South, West, None,
}

#[derive(Component)]
struct PlayerAttackDirection {
    direction:Direction,
}

#[derive(Component)]
struct CreatureAttackDirection {
    direction:Direction,
}

#[derive(Component)]
struct MovementTimer(Timer);

#[derive(Resource)]
struct OneSecondTimer(Timer);

#[derive(Resource)]
struct TwoSecondTimer(Timer);

#[derive(Resource)]
struct FiveSecondTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Assets<Image>>) {
    let texture_handle_grass1 = asset_server.load("grass1.png");
    let texture_handle_grass2 = asset_server.load("grass2.png");
    let texture_handle_grass3 = asset_server.load("grass3.png");
    let texture_handle_grass4 = asset_server.load("grass4.png");
    let texture_handle_water = asset_server.load("water.png");
    let texture_handle_blob = asset_server.load("blobplayer.png");
    let texture_handle_rock = asset_server.load("rock.png");

    commands.spawn((
            Player,
            ObstaclePos { xgrid: 0.0, ygrid: 0.0 },
            OnMap,
            PlayerAttackDirection {direction: Direction::None},
        GridPos { xgrid: 0.0, ygrid: 0.0 },
        UpdateGridPos { xgrid: 0.0, ygrid: 0.0 },
        SpriteBundle {
            texture: texture_handle_blob.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
    ));

fn spawn_rock(
    commands: &mut Commands,
    texture_handle_rock: Handle<Image>,
    spawn_xgrid: f32,
    spawn_ygrid: f32,
    )
    {commands.spawn((
        OnMap,
        GridPos { xgrid: 0.0, ygrid: 0.0 },
        Health { health: 40, max_health: 40},
        Alive { remove: false },
        ObstaclePos { xgrid: 0.0, ygrid: 0.0 },
        UpdateGridPos { xgrid: spawn_xgrid, ygrid: spawn_ygrid },
        SpriteBundle {
            texture: texture_handle_rock.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
    ));}
    spawn_rock(&mut commands, texture_handle_rock.clone(), 1.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 2.0, 5.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 2.0, 7.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 2.0, 9.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 3.0, 1.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 3.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 3.0, 6.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 2.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 4.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 8.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 5.0, 2.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 5.0, 9.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 7.0, 5.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 7.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 8.0, 6.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 9.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 12.0, 7.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 12.0, 9.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 13.0, 1.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 13.0, 3.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 13.0, 6.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 12.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 14.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 4.0, 13.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 5.0, 12.0);
    spawn_rock(&mut commands, texture_handle_rock.clone(), 5.0, 16.0);

// Example 2D array for the grid (0 for empty, 1 for grass)
    let mut rng = rand::thread_rng();
    let mut grid: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [
        [0; GRID_WIDTH]; GRID_HEIGHT];

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            grid[y][x] = rng.gen_range(1..=4); }}

    // Spawn the sprites based on the grid
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
          match grid[y][x] {
                1 => spawn_tile(&mut commands, texture_handle_grass1.clone(), x, y),
                2 => spawn_tile(&mut commands, texture_handle_grass2.clone(), x, y),
                3 => spawn_tile(&mut commands, texture_handle_grass3.clone(), x, y),
                4 => spawn_tile(&mut commands, texture_handle_grass4.clone(), x, y),
                5 => spawn_tile(&mut commands, texture_handle_water.clone(), x, y),
                _ => {}
            }
        }
    }



    commands.spawn(Camera2dBundle::default());
}



fn resize_camera(
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut resize_events: EventReader<WindowResized>,
) {
    if !resize_events.is_empty() {
    let mut projection = query.single_mut();
    let window = windows.single();

    let aspect_ratio = window.width() / window.height();
    let target_aspect_ratio = 320.0 / 180.0;
if aspect_ratio > target_aspect_ratio {
        projection.scaling_mode = ScalingMode::FixedVertical(180.0);
    } else {
        projection.scaling_mode = ScalingMode::FixedHorizontal(320.0);
    }
    }
}

//Potential problem! Later when entities don't have a ObstaclePos this won't work
//Perhaps make fn with without ObstaclePos
fn update_pos(
    mut query: Query<(&mut GridPos, &mut ObstaclePos, &UpdateGridPos)>,
    ) {
    for (mut GridPos, mut ObstaclePos, UpdateGridPos) in &mut query {
        GridPos.xgrid = UpdateGridPos.xgrid;
        GridPos.ygrid = UpdateGridPos.ygrid;
        ObstaclePos.xgrid = GridPos.xgrid;
        ObstaclePos.ygrid = GridPos.ygrid;
            }

}

fn move_entities(mut query: Query<(&GridPos, &mut Transform), With<OnMap>>) {
    for (GridPos, mut transform) in &mut query {
       let new_x = (GridPos.xgrid * CELL_SIZE) - (SCREEN_WIDTH / 2.0) + 5.0;
       let new_y = (GridPos.ygrid * CELL_SIZE) - (SCREEN_HEIGHT / 2.0) + 5.0;

        transform.translation.x = new_x;
        transform.translation.y = new_y;
    };
}

fn health_check(
mut query: Query<(&mut Alive, &Health)>
    ) {
    for (mut Alive, Health) in query.iter_mut() {
        if Health.health <= 0 { Alive.remove = true; }
    }
}

fn remove_dead(
    mut commands: Commands, query: Query<(Entity, &Alive)>
    ) {
for (entity, Alive) in query.iter() {
    if Alive.remove {
        commands.entity(entity).despawn();
    }
}
}
//make events for movement and attack?
fn keyboard_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&GridPos, &mut UpdateGridPos, &mut PlayerAttackDirection), With<Player>>,
    obstacle_query: Query<&ObstaclePos>,
) {
    if keys.get_pressed().next().is_some() {
        let mut blocked = false;
        let mut new_x = 0.0;
        let mut new_y = 0.0;
    for (GridPos, mut UpdateGridPos, mut PlayerAttackDirection) in &mut query {
        UpdateGridPos.xgrid = GridPos.xgrid;
        UpdateGridPos.ygrid = GridPos.ygrid;

            PlayerAttackDirection.direction = Direction::None;
        if keys.pressed(KeyCode::KeyZ) {
            if keys.just_pressed(KeyCode::ArrowUp) {PlayerAttackDirection.direction = Direction::North}
            else if keys.just_pressed(KeyCode::ArrowDown) {PlayerAttackDirection.direction = Direction::South}
            else if keys.just_pressed(KeyCode::ArrowLeft) {PlayerAttackDirection.direction = Direction::West}
            else if keys.just_pressed(KeyCode::ArrowRight) {PlayerAttackDirection.direction = Direction::East}

        }
        else
            if keys.just_pressed(KeyCode::ArrowUp) {
               if UpdateGridPos.ygrid < F_GRID_HEIGHT - 1.0 {UpdateGridPos.ygrid += 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowDown) {
               if UpdateGridPos.ygrid > 0.0 {UpdateGridPos.ygrid -= 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowLeft) {
               if UpdateGridPos.xgrid > 0.0 {UpdateGridPos.xgrid -= 1.0;}
            } else if keys.just_pressed(KeyCode::ArrowRight) {
               if UpdateGridPos.xgrid < F_GRID_WIDTH - 1.0 {UpdateGridPos.xgrid += 1.0;}
            } 
        new_x = UpdateGridPos.xgrid;
        new_y = UpdateGridPos.ygrid;
            for ObstaclePos in &obstacle_query {
            if  new_x == ObstaclePos.xgrid && new_y ==  ObstaclePos.ygrid {
                blocked = true;
                break;
        }
    }

        // If blocked, update the position to previous position
        if blocked {
            UpdateGridPos.xgrid = GridPos.xgrid;
            UpdateGridPos.ygrid = GridPos.ygrid;
        }
            }
        }
}
fn create_player_attack(
    mut query: Query<(&GridPos, &mut PlayerAttackDirection), With<Player>>,
     mut commands: Commands,
asset_server: Res<AssetServer>,
            ) {
//BUG FIX: make it so you can aim off the screen
    let texture_handle_attack = asset_server.load("attack.png");
    for (GridPos, mut PlayerAttackDirection) in query.iter_mut() {
   let mut attack_x = GridPos.xgrid; 
   let mut attack_y = GridPos.ygrid; 
   match
       PlayerAttackDirection.direction {
Direction::North => { attack_y += 1.0;}
Direction::South => { attack_y -= 1.0;}
Direction::East => { attack_x += 1.0;}
Direction::West => { attack_x -= 1.0;}
Direction::None => {}
       }
   if PlayerAttackDirection.direction != Direction::None
    {commands.spawn((
AttackImage {
    time_visual: Timer::from_seconds(0.4, TimerMode::Once),
    remove: false,
    xgrid: attack_x,
    ygrid: attack_y,
},

GridPos {
    xgrid: attack_x,
    ygrid: attack_y,
},

AttackDamage {
    attack_damage: 20,
    remove: false,
    xgrid: attack_x,
    ygrid: attack_y,
},
OnMap,

        SpriteBundle {
            texture: texture_handle_attack.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..Default::default()
        },
    ));}
    //direction enum(change back to None after)
}

}
fn create_creature_attack(
    mut query: Query<(&GridPos, &mut CreatureAttackDirection, &mut JustMoved), With<Creature>>,
     mut commands: Commands,
asset_server: Res<AssetServer>,
            ) {
//BUG FIX: make it so you can aim off the screen
    let texture_handle_fire_attack = asset_server.load("fire_attack.png");
    for (GridPos, mut CreatureAttackDirection, mut JustMoved) in query.iter_mut() {
    if JustMoved.just_moved {
   let mut attack_x = GridPos.xgrid; 
   let mut attack_y = GridPos.ygrid; 
   match
       CreatureAttackDirection.direction {
Direction::North => { attack_y -= 1.0;}
Direction::South => { attack_y += 1.0;}
Direction::East => { attack_x -= 1.0;}
Direction::West => { attack_x += 1.0;}
Direction::None => {}
       }
   if CreatureAttackDirection.direction != Direction::None
    {commands.spawn((
AttackImage {
    time_visual: Timer::from_seconds(0.4, TimerMode::Once),
    remove: false,
    xgrid: attack_x,
    ygrid: attack_y,
},

GridPos {
    xgrid: attack_x,
    ygrid: attack_y,
},

AttackDamage {
    attack_damage: 20,
    remove: false,
    xgrid: attack_x,
    ygrid: attack_y,
},
OnMap,

        SpriteBundle {
            texture: texture_handle_fire_attack.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..Default::default()
        },
    ));}
    //direction enum(change back to None after)
    CreatureAttackDirection.direction = Direction::None;
}
    JustMoved.just_moved = false;
}
}


//Make it remove that component not whole entity!
//Removes only attack damage component
fn remove_attack_damage(
    mut commands: Commands, mut query: Query<(Entity, &mut AttackDamage)>
    ) {
for (entity, mut AttackDamage) in query.iter_mut() {
    if AttackDamage.remove  {
            commands.entity(entity).remove::<AttackDamage>();
    }
}
}

//Removes whole attack entity!
fn remove_attack_image(
    mut commands: Commands, mut query: Query<(Entity, &mut AttackImage,)>, time: Res<Time>,
    ) {
for (entity, mut AttackImage) in query.iter_mut() {
    AttackImage.time_visual.tick(time.delta());
    if AttackImage.time_visual.finished() {AttackImage.remove = true;}
    if AttackImage.remove {
        commands.entity(entity).despawn();
    }
}
}
//Change AttackDamage.remove to true after
                // Health change event?
fn apply_damage(
    mut health_query: Query<(&mut Health, &GridPos)>,
    mut attack_query: Query<(&mut AttackDamage, &GridPos)>,
) {
    for (mut attack_damage, attack_pos) in attack_query.iter_mut() {
        // Iterate over all entities with Health and GridPos components
        for (mut health, health_pos) in health_query.iter_mut() {
            // Check if the attack entity's position matches the health entity's position
            if attack_pos.xgrid == health_pos.xgrid && attack_pos.ygrid == health_pos.ygrid {
                // Apply damage to the health component
                health.health -= attack_damage.attack_damage;

                // Ensure health doesn't go below 0
                if health.health < 0 {
                    health.health = 0;
                }

                // Mark the attack for removal after applying damage
                attack_damage.remove = true;

                // Since the attack is targeted at a single entity, we can break the loop once damage is applied
                break;
            }
        }
    }
}
fn timed_spawner(
time: Res<Time>,
mut timer: ResMut<FiveSecondTimer>,
    mut commands: Commands,
asset_server: Res<AssetServer>,
    )
{
if timer. 0.tick(time.delta()).just_finished() {
        let texture_handle_fire_critter = asset_server.load("firecritter.png");
    println!("Boop: spawn enemy");
    spawn_fire_critter(&mut commands, texture_handle_fire_critter.clone(), 5.0, 3.0)
}
}

//Idea: Make a generic enemy move function and add components to enemy entities to describe their
//movement, making the movement modular, aka run away movement, aggressive, passive, random,
//aggressive range X, run away hp X, enable charge (move 2 squares at once)T/F, ect
fn timed_movement(
time: Res<Time>,
mut timer: ResMut<OneSecondTimer>,
mut enemy_query: Query<(&mut GridPos, &mut UpdateGridPos, &mut JustMoved), (With<Creature>, Without<Player>)>,
mut player_query: Query<&GridPos, (With<Player>, Without<Creature>)>,
    obstacle_query: Query<&ObstaclePos>,
    )
{
if timer. 0.tick(time.delta()).just_finished() {
    println!("Blop: Creature MOVE");
    enum UpDown {
    Up,
    Down,
    None,
    }
    enum LeftRight {
    Left,
    Right,
    None,
    }
    let mut creature_updown = UpDown::None;
    let mut creature_leftright = LeftRight::None;
    let mut temp_x = 0.0;
    let mut temp_y = 0.0;

    for GridPos in player_query.iter() {
temp_x = GridPos.xgrid;
temp_y = GridPos.ygrid;
    }

    for (mut GridPos, mut UpdateGridPos, mut JustMoved) in enemy_query.iter_mut() {
        if GridPos.xgrid != temp_x {
            if GridPos.xgrid > temp_x {creature_leftright = LeftRight::Right}
            else {creature_leftright = LeftRight::Left};
        }
        if GridPos.ygrid != temp_y {
            if GridPos.ygrid > temp_y {creature_updown = UpDown::Up}
            else {creature_updown = UpDown::Down};
        }
        match creature_leftright {
    LeftRight::None => println!("Brip: Creature didn't move left/right"),
    LeftRight::Right => {UpdateGridPos.xgrid = UpdateGridPos.xgrid - 1.0}
    LeftRight::Left => {UpdateGridPos.xgrid = UpdateGridPos.xgrid + 1.0}
        }

         match creature_updown {
    UpDown::None => println!("Brep: Creature didn't move up/down"),
    UpDown::Up => {UpdateGridPos.ygrid = UpdateGridPos.ygrid - 1.0}
    UpDown::Down => {UpdateGridPos.ygrid = UpdateGridPos.ygrid + 1.0}
       }
        JustMoved.just_moved = true;
        let mut blocked = false;
        let mut new_x = 0.0;
        let mut new_y = 0.0;
        new_x = UpdateGridPos.xgrid;
        new_y = UpdateGridPos.ygrid;
            for ObstaclePos in &obstacle_query {
            if  new_x == ObstaclePos.xgrid && new_y ==  ObstaclePos.ygrid {
                blocked = true;
                break;
        }
    }

        // If blocked, update the position to previous position
        if blocked {
            UpdateGridPos.xgrid = GridPos.xgrid;
            UpdateGridPos.ygrid = GridPos.ygrid;
            //Try again on X axis

                blocked = false;
        match creature_leftright {
    LeftRight::None => {}
    LeftRight::Right => {UpdateGridPos.xgrid = UpdateGridPos.xgrid - 1.0}
    LeftRight::Left => {UpdateGridPos.xgrid = UpdateGridPos.xgrid + 1.0}
        }

        new_x = UpdateGridPos.xgrid;
        new_y = UpdateGridPos.ygrid;
            for ObstaclePos in &obstacle_query {
            if  new_x == ObstaclePos.xgrid && new_y ==  ObstaclePos.ygrid {
                blocked = true;
                break;
        }
    }

        // If blocked, update the position to previous position
        if blocked {
            UpdateGridPos.xgrid = GridPos.xgrid;
            UpdateGridPos.ygrid = GridPos.ygrid;
            //Try again on Y axis

                blocked = false;
         match creature_updown {
    UpDown::None => println!("Brep: Creature didn't move up/down"),
    UpDown::Up => {UpdateGridPos.ygrid = UpdateGridPos.ygrid - 1.0}
    UpDown::Down => {UpdateGridPos.ygrid = UpdateGridPos.ygrid + 1.0}
       }

        new_x = UpdateGridPos.xgrid;
        new_y = UpdateGridPos.ygrid;
            for ObstaclePos in &obstacle_query {
            if  new_x == ObstaclePos.xgrid && new_y ==  ObstaclePos.ygrid {
                blocked = true;
                break;
        }
    }

        // If blocked, update the position to previous position
        if blocked {
            UpdateGridPos.xgrid = GridPos.xgrid;
            UpdateGridPos.ygrid = GridPos.ygrid;
            //Failed to move
            println!("Bulp: Fire Critter failed to move!")
        }

        }

        }

         creature_updown = UpDown::None;
         creature_leftright = LeftRight::None;
        //use local enum for up, down, none, and left, right, none, then use match statement to
        //decide direction
println!("Blep: query: x = {}, y = {}",temp_x, temp_y);
//if GridPos.xgrid != temp_x {if GridPos.xgrid > temp_x output xgrid -1, else xgrid +1}
//maybe make left right, up down enums to capture relative location of player? Too complex?
//Maybe just output location x + 1, x, x -1, ect
//create a random function to move in one of two axis
//Also think of obstacle collision, ect
    }
 
}
}

fn timed_attack(
time: Res<Time>,
mut timer: ResMut<TwoSecondTimer>,
mut enemy_query: Query<(&mut GridPos, &mut CreatureAttackDirection, &mut JustMoved), (With<Creature>, Without<Player>)>,
mut player_query: Query<&GridPos, (With<Player>, Without<Creature>)>,
    )
{
if timer. 0.tick(time.delta()).just_finished() {
    println!("Blup: Creature ATTACk");
    enum UpDown {
    Up,
    Down,
    None,
    }
    enum LeftRight {
    Left,
    Right,
    None,
    }
    let mut creature_updown = UpDown::None;
    let mut creature_leftright = LeftRight::None;
    let mut temp_x = 0.0;
    let mut temp_y = 0.0;

    for GridPos in player_query.iter() {
temp_x = GridPos.xgrid;
temp_y = GridPos.ygrid;
    }

    for (mut GridPos, mut CreatureAttackDirection, mut JustMoved) in enemy_query.iter_mut() {
        if GridPos.xgrid != temp_x {
            if GridPos.xgrid > temp_x {creature_leftright = LeftRight::Right}
            else {creature_leftright = LeftRight::Left};
        }
        if GridPos.ygrid != temp_y {
            if GridPos.ygrid > temp_y {creature_updown = UpDown::Up}
            else {creature_updown = UpDown::Down};
        }
        match creature_updown {
    UpDown::None => println!("Bvep: Creature didn't attack up/down"),
    UpDown::Up => {CreatureAttackDirection.direction = Direction::North}
    UpDown::Down => {CreatureAttackDirection.direction = Direction::South}
       }
        match creature_leftright {
    LeftRight::None => println!("Bvip: Creature didn't attack left/right"),
    LeftRight::Right => {CreatureAttackDirection.direction = Direction::East}
    LeftRight::Left => {CreatureAttackDirection.direction = Direction::West}
        }

         creature_updown = UpDown::None;
         creature_leftright = LeftRight::None;
         JustMoved.just_moved = false;
    }
 
}
}
// Starting to feel it's time for a redesign, it's doing a lot more work than needed.
// Use events to cut down on work, e.g. when keypressed trigger keyboard input fn, when x moves,
// when x attacks ect
fn spawn_fire_critter(
    commands: &mut Commands,
    texture_handle_fire_critter: Handle<Image>,
    spawn_xgrid: f32,
    spawn_ygrid: f32,
    )
    {commands.spawn((
        OnMap,
        Creature,
        JustMoved { just_moved: false },
        GridPos { xgrid: 0.0, ygrid: 0.0 },
        Health { health: 20, max_health: 20},
        Alive { remove: false },
        CreatureAttackDirection {direction: Direction::None},
        ObstaclePos { xgrid: 0.0, ygrid: 0.0 },
        UpdateGridPos { xgrid: spawn_xgrid, ygrid: spawn_ygrid },
        SpriteBundle {
            texture: texture_handle_fire_critter.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
    ));}
