pub use crate::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_startup_system(initialize_animation_controllers.system().label("Initialize Components"))
      .add_system(animate_sprite_system.system());
  }
}
