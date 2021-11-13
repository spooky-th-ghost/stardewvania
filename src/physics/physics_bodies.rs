use crate::prelude::*;

pub struct PhysBody {
  velocity: Vec2,
  force: PhysForce
}

impl PhysBody {
  pub fn new() -> Self {
    PhysBody {
      velocity: Vec2::ZERO,
      force: PhysForce::new(Vec2::ZERO)
    }
  }

  pub fn add_force(&mut self, other: PhysForce) {
    self.force.add(other);
  }
  pub fn mul_force(&mut self, other: PhysForce) {
    self.force.mul(other);
  }

  pub fn update_velocity(&mut self) {
    self.velocity = self.force.force;
  }
}

pub fn apply_velocity(
  time: Res<Time>,
  mut query: Query<(&mut Transform, &PhysBody)>,
) {
  for (mut transform, body) in query.iter_mut() {
    transform.translation += Vec3::new(body.velocity.x, body.velocity.y, 0.0) * time.delta_seconds();
  }
}
