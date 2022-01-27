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
        .add_plugin(PhysicsPlugin)
        .add_system(player_movement.system())
        .add_system(update_anim_variables.system())
        .run();
}


fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut PhysBody, &mut Transform)>
) {
    if let Ok((mut player, mut body, mut transform)) = query.single_mut() {
        let mut move_force = Vec2::ZERO;
        
        if input.pressed(KeyCode::S) {
            player.direction = 2;
            player.state = AnimState::Run;
            move_force.y = -1.0;
            
        }

        if input.pressed(KeyCode::W) {
            player.direction = 8;
            player.state = AnimState::Run;
            move_force.y = 1.0;
        }

        if input.pressed(KeyCode::A) {
            player.direction = 4;
            player.state = AnimState::Run;
            move_force.x = -1.0;
        }

        if input.pressed(KeyCode::D) {
            player.direction = 6;
            player.state = AnimState::Run;
            move_force.x = 1.0;
        }

        transform.translation.z += 0.125;
        println!("Transform: {:?}", transform.translation);
        

        if move_force != Vec2::ZERO {
            body.add_force(move_force.normalize());
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

    let mut camera = PerspectiveCameraBundle::new_3d();
    camera.perspective_projection.fov = 20.0;
    //camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(0.0, 25.0, -50.0).looking_at(Vec3::ZERO, Vec3::Y);
    coms.spawn_bundle(camera);
    coms.spawn_bundle(UiCameraBundle::default());
    coms
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player::new())
        .insert(AnimationController::new(String::from("player")))
        .insert(PhysBody::new());
    

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

