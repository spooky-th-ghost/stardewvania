use crate::prelude::*;

pub struct PhysBody {
  position: Vec2,
  velocity: Vec2,
  force: Vec2,
  max_speed: f32,
}

impl PhysBody {
  pub fn new() -> Self {
    PhysBody {
      position: Vec2::ZERO,
      velocity: Vec2::ZERO,
      force: Vec2::ZERO,
      max_speed: 200.0
    }
  }

  pub fn add_force(&mut self, other: Vec2) {
    self.force += other;
  }
  pub fn mul_force(&mut self, other: Vec2) {
    self.force *= other;
  }

  pub fn update_velocity(&mut self) {
    if self.force != Vec2::ZERO {
      self.velocity += self.force;
    }
  }

  pub fn decay(&mut self, t: f32) {
    self.velocity.lerp(Vec2::ZERO, t);
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

pub fn update_velocity(
  time: Res<Time>,
  mut query: Query<&mut PhysBody>,
) {
  for mut body in query.iter_mut() {
    //body.update_velocity();
    println!("Velocity: {:?}", body.velocity);
    println!("Force: {:?}", body.force);
    //body.decay(time.delta_seconds());
    // if body.velocity.x.abs() >= body.max_speed {
    //   body.decay(time.delta_seconds());
    // } else {
    //   body.update_velocity();
    // }
  }
}
