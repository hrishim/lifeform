use bevy::prelude::*;

/*

Every creature has
- A type
- Health (also sets size)
- Speed 1-10. Relevant only in moving state
- Move cost, Wait cost, Scan cost
- Powers: Scan, Scan range
- A state: Moving, waiting, scanning.
- Walk animation path
- Wait animation path
- Die animation path
- Current location

*/

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_algae)
        .add_startup_system(setup_carnivore)
        .add_startup_system(setup_creatures)
        .add_system(animate_sprite)
        .run();
}


#[derive(Default)]
struct GameState {
    num_creatures: usize,
    num_algae: usize,
}

////////////////////////////////
///         Components       ///
////////////////////////////////

#[derive(Component)]
enum Creature {
    Algae,
    Herbivore,
    Carnivore,
    Omnivore,
    Cannibal,
}

#[derive(Component)]
enum CreatureState {
    Idle,
    Moving(u32), // holds speed value from 1-10
    Scanning,
    Divide
}

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct MoveCost(u32);

#[derive(Component)]
struct WaitCost(u32);

#[derive(Component)]
struct ScanCost(u32);

#[derive(Component)]
struct ScanRange(u32);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);



// system function to add algae
fn add_flora(
    time: Res<GameState>,
 ) {

}

// system function to setup algae
fn setup_algae(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("textures/tree-ornament.png"),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Health(20))
        .insert(Creature::Algae);
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn setup_carnivore(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    //Load a sprite sheet image file
    //let texture_handle = asset_server.load("textures/mon1_sprite.png");
    let texture_handle = asset_server.load("textures/bat_walk.png");
    //From that sheet get the specific sprites (specified by row and coumns in that row.)
    //let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 5, 1);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 41.0), 11, 1);
    let texture_atlas_handle_move = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_move,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Health(20))
        .insert(Creature::Carnivore);
}


fn setup_creatures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    /*
    //Load a sprite sheet image file
    let texture_handle = asset_server.load("textures/mon1_sprite.png");

    //From that sheet get the specific sprites (specified by row and coumns in that row.)
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 5, 1);
    let texture_atlas_handle_move = texture_atlases.add(texture_atlas);

    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle_move,
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..default()
    })
    .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
    .insert(Health(20))
    .insert(Creature::Herbivore);

    */

    let texture_handle = asset_server.load("textures/mon1_sprite.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 4, 2);
    let texture_atlas_handle_attack = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("textures/mon1_sprite.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 5, 3);
    let texture_atlas_handle_die = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("textures/water.jpg");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 50.0), 3, 3);
    let texture_atlas_handle_algae = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //setup_carnivore(commands, asset_server, texture_atlases);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_attack,
            transform: Transform::from_scale(Vec3::splat(1.0))
                .with_translation(Vec3::new(-100.0, 0.0, 0.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_die,
            transform: Transform::from_scale(Vec3::splat(1.0))
                .with_translation(Vec3::new(100.0, 0.0, 0.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

// https://bevouliin.com/category/free_game_asset/
// https://opengameart.org/content/library-of-2d-game-art
// https://codeshack.io/images-sprite-sheet-generator/