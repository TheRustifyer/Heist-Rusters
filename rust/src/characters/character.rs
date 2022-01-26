use gdnative::prelude::*;

/// Using the library `https://github.com/Pyzyryab/gdrust-helper`
/// that will allow me to reduce the cognitive complexity of some
/// common operations of godot-rust
use gdrusthelper as gdrust;
use gdrust::gdmotion::KeysMotionMouseDirection;

use crate::utils::constants::in_game_constant::{
    CHARACTER_CONFIGURATION,
    MOTION_KEYBINDINGS
};

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[derive(Debug)]
pub struct Character { 
    // Tracks the movement of the player
    motion: Vector2
}

impl KeysMotionMouseDirection for Character { }

#[gdnative::prelude::methods]
impl Character {
    pub fn new(_owner: &KinematicBody2D) -> Self { 
        Self { 
            motion: Vector2::ZERO
        }
    }

    #[export]
    fn _ready(&self, owner: &KinematicBody2D) {
        // Just for debug the child nodes
        for child in owner.get_children().into_iter() {
            godot_print!("Node: {:?}", &child);
        }
    }

    #[export]
    fn _physics_process(&self, owner: &KinematicBody2D, _delta: f32) {
        let motion: Vector2 = self.move_character(
            owner,
            self.motion,
            CHARACTER_CONFIGURATION,
            MOTION_KEYBINDINGS
        );
        owner.move_and_slide(
            motion, 
            Vector2::new(0.0, 0.0), 
            false, 
            4 as i64, 
            0.785398 as f64, 
            true
        );
    }

}