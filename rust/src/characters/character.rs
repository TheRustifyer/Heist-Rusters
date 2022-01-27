use gdnative::{prelude::*, api::Light2D};

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
    motion: Vector2,
    // The status (ON/OFF) of the player's lantern
    lantern: Option<TRef<'static, Light2D>>
}

impl KeysMotionMouseDirection for Character { }

#[gdnative::prelude::methods]
impl Character {
    pub fn new(_owner: &KinematicBody2D) -> Self { 
        Self { 
            motion: Vector2::ZERO,
            lantern: None
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        // Recovering the node of the torch light
        self.lantern = unsafe { owner
            .get_child(2)
            .unwrap()
            .assume_safe()
            .cast::<Light2D>()
        };
    }

    #[export]
    fn _physics_process(&self, owner: &KinematicBody2D, _delta: f32) {
        let motion: Vector2 = self.move_character(
            owner,
            self.motion,
            CHARACTER_CONFIGURATION,
            MOTION_KEYBINDINGS
        );
        // For the sake of simplicity, we will set here the action that will switch ON/OFF
        // the player's torch. NOTE: `.get_child(idx: 2)` -> Two it's the current index of the node
        // relative to the player's KinematicBody2D root node
        if Input::is_action_just_pressed(
            Input::godot_singleton(), "torch_toggle", false
        ) { 
            self.lantern.unwrap().set_enabled(
                !self.lantern.unwrap().is_enabled()
            );
        }
        
        // Move the player with the updated inputted data
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