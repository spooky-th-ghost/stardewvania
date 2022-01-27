use crate::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
        //.add_system(apply_velocity.system())
        .add_system(update_velocity.system());
  }
}
