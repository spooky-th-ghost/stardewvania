pub use crate::prelude::*;

#[derive(Clone,Copy, PartialEq, Debug)]
pub enum AnimState {
  Run,
  Idle
}

pub struct AnimationLibrary {
  animations: HashMap<String, Animation>
}

impl AnimationLibrary {
  pub fn new(animations: HashMap<String, Animation>) -> Self {
    AnimationLibrary {
      animations
    }
  }

  pub fn get(&self, anim_id: String) -> Option<Animation>{
    if let Some(animation) = self.animations.get(&anim_id) {
      return Some(animation.clone());
    } else {
      return None;
    }
  }
}

#[derive(Clone,Copy, PartialEq, Debug)]
pub struct Animation {
  pub first_frame: usize,
  pub length: usize,
  pub loops: bool,
}

impl Animation {
  pub fn new(first_frame: usize, length: usize, loops: bool) -> Self {
    Animation {
      first_frame,
      length,
      loops
    }
  }
  pub fn transform_index(&self, relative_index: usize) -> (usize,usize) {
    let new_true_index: usize;
    let mut new_relative_index: usize = relative_index + 1;
    if new_relative_index >= self.length {
      new_true_index = self.first_frame;
      new_relative_index = 0;
    } else {
      new_true_index = self.first_frame + new_relative_index;
    }
    println!("Relative: {0}, True: {1}", new_relative_index, new_true_index);
    return (new_relative_index, new_true_index);
  }
}

#[derive(Debug)]
pub struct AnimationController {
  character_prefix: String,
  anim_state: AnimState,
  direction: u8,
  pub current_animation: Option<Animation>,
  relative_index: usize
}

impl AnimationController {
  pub fn new(character_prefix: String) -> Self {
    AnimationController {
      character_prefix,
      anim_state: AnimState::Idle,
      direction: 1,
      current_animation: None,
      relative_index: 0
    }
  }

  pub fn get_next_frame(&mut self) -> usize {
    if let Some(animation) = self.current_animation {
      let (new_relative_index, true_index) = animation.transform_index(self.relative_index);
      self.relative_index = new_relative_index;
      return true_index;
    } else {
      self.relative_index = 0;
      return 0;
    }
  }

  pub fn set_direction(&mut self, library: &AnimationLibrary, new_direction: u8) {
    if self.direction != new_direction {
      self.direction = new_direction;
      let state_str = match self.anim_state {
        AnimState::Idle => "idle",
        AnimState::Run => "run"
      };
      self.current_animation = library.get(format!("{0}_{1}_{2}", self.character_prefix,state_str, self.direction));
    }
  }

  pub fn set_anim_state(&mut self, library: &AnimationLibrary, new_state: AnimState) {
    if self.anim_state != new_state {
      self.anim_state = new_state;
      let state_str = match self.anim_state {
        AnimState::Idle => "idle",
        AnimState::Run => "run"
      };
      self.current_animation = library.get(format!("{0}_{1}_{2}", self.character_prefix,state_str, self.direction));
    }
  }

  pub fn get_initial_animation(&mut self, library: &AnimationLibrary) {
      let state_str = match self.anim_state {
        AnimState::Idle => "idle",
        AnimState::Run => "run"
      };
      let anim_id = format!("{0}_{1}_{2}", self.character_prefix,state_str, self.direction);
      println!("{}",anim_id);
      self.current_animation = library.get(anim_id);
    }
  
}

pub fn initialize_animation_controllers(
  mut coms: Commands,
  mut query: Query<&mut AnimationController>,
) {
  let library =
          AnimationLibrary::new(
          HashMap::from_iter::<HashMap<String, Animation>>(
            [
              (String::from("player_idle_2"), Animation::new(64, 8, false)),
              (String::from("player_idle_1"), Animation::new(72, 8, false)),
              (String::from("player_idle_4"), Animation::new(80,8, false)),
              (String::from("player_idle_7"), Animation::new(88, 8, false)),
              (String::from("player_idle_8"), Animation::new(96,8, false)),
              (String::from("player_idle_9"), Animation::new(104, 8, false)),
              (String::from("player_idle_6"), Animation::new(112,8, false)),
              (String::from("player_idle_3"), Animation::new(120,8, false)),
              (String::from("player_run_2"), Animation::new(0,8, false)),
              (String::from("player_run_1"), Animation::new(8,8, false)),
              (String::from("player_run_4"), Animation::new(16,8, false)),
              (String::from("player_run_7"), Animation::new(24,8, false)),
              (String::from("player_run_8"), Animation::new(32,8, false)),
              (String::from("player_run_9"), Animation::new(40,8, false)),
              (String::from("player_run_6"), Animation::new(48,8, false)),
              (String::from("player_run_3"), Animation::new(56,8, false)),
            ].iter().cloned().collect()
          )
        );
  for mut controller in query.iter_mut() {
    println!("Found one");
    controller.get_initial_animation(&library);
  }
  coms.insert_resource(library);
  println!("Initialized Animation Controllers");
}

 pub struct AnimationPlugin;

  impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
      app
        .add_startup_system(initialize_animation_controllers.system().label("Initialize Components"));
    }
  }
