pub mod animation;
pub mod physics;

mod prelude {
    pub use bevy::{
        prelude::*,
        utils::HashMap,
    };
    pub use std::{
        collections::hash_map::IntoIter,
        fmt,
        cmp::PartialEq,
    };
    pub use crate::animation::*;
    pub use crate::physics::*;
}
use crate::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system().before("Initialize Components"))
        .add_plugin(AnimationPlugin)
        .add_system(player_movement.system())
        .add_system(update_anim_variables.system())
        .run();
}


fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Player>
) {
    if let Ok(mut player) = query.single_mut() {

        if input.pressed(KeyCode::S) {
            player.direction = 2;
            player.state = AnimState::Run;
            return;
        }

        if input.pressed(KeyCode::W) {
            player.direction = 8;
            player.state = AnimState::Run;
            return;
        }

        if input.pressed(KeyCode::A) {
            player.direction = 4;
            player.state = AnimState::Run;
            return;
        }

        if input.pressed(KeyCode::D) {
            player.direction = 6;
            player.state = AnimState::Run;
            return;
        }
            

        player.state = AnimState::Idle;
    }
}

fn update_anim_variables(
    library: Res<AnimationLibrary>,
    mut query: Query<(&Player,&mut AnimationController)>,
) {
    for (player,mut anim) in query.iter_mut() {
        anim.set_direction(&library,player.direction);
        anim.set_anim_state(&library,player.state);
    }
}



fn setup(
    mut coms: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    println!("Insert Components");
    let texture_handle = asset_server.load("sprites/robot.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0,64.0), 8, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    coms.spawn_bundle(OrthographicCameraBundle::new_2d());
    coms.spawn_bundle(UiCameraBundle::default());
    coms
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player::new())
        .insert(AnimationController::new(String::from("player")));
    
    println!("Inserted Animation Controller");

}

pub struct Player {
    pub direction: u8,
    pub state: AnimState,
}

impl Player{
    pub fn new() -> Self {
        Player {
            direction: 2,
            state: AnimState::Idle,
        }
    }
}

