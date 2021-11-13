use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct PhysForce {
  pub force: Vec2,
}

impl PhysForce {
  pub fn new(force: Vec2) -> Self {
    PhysForce {
      force
    }
  }

  pub fn mul(&mut self, other: PhysForce) {
    self.force *= other.force;
  }

  pub fn add(&mut self, other: PhysForce) {
    self.force += other.force;
  }

  pub fn subtract(&mut self, other: PhysForce) {
    self.force -= other.force;
  }

  pub fn decay(&mut self, t: f32) {
    self.force.lerp(Vec2::ZERO, t);
  }
}

pub fn decay_force(
  time: Res<Time>,
  mut query: Query<&mut Transform, Changed<PhysBody>>,
) {

}
