use gdnative::prelude::*;
use gdrusthelper as gdrust;

use crate::characters::motion::Motion2DWithMouse;

use crate::utils::game_constants::in_game_constant::{
    CHARACTER_CONFIGURATION
};

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[derive(Debug)]
pub struct Character {
    // Binding to the Input singleton
    input: Option<&'static Input>,
    // Tracks the movement of the player
    motion: Vector2
}

/// Implements the `Motion2DWithMouse` trait that already provides an implementation
/// for move the character with the keyboard define ones and aiming the facing direction
/// with the mouse
impl Motion2DWithMouse for Character { }

#[gdnative::methods]
impl Character {
    pub fn new(_owner: &KinematicBody2D) -> Self { 
        Self { 
            // Input 
            input: Some(Input::godot_singleton()),
            // Player motion
            motion: Vector2::zero()
        }
    }

    #[export]
    fn _ready(&self, owner: &KinematicBody2D) {
        // Using the library `https://github.com/Pyzyryab/gdrust-helper`
        // that will allow me to reduce the cognitive complexity of some
        // common operations of godot-rust
        gdrust::health_check_from_github();
        
        // Creates a new sprite node with an GFX asset already loaded
        let sprite = gdrust::gdcreator::sprite_with_asset(
            "sprite_child","assets/GFX/PNG/Man Red/manRed_stand.png"
        );
        sprite.set_visible(true);
        owner.add_child(sprite, true);
        // Setting the player's starting point on the map
        owner.set_position(Vector2::new(300.0, 200.0));
        
        for child in owner.get_children().into_iter() {
            godot_print!("Node: {:?}", &child);
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        let mut motion = self.motion;
        self.move_character(owner, self.input, &mut motion, CHARACTER_CONFIGURATION);
        self.motion = motion;
        owner.move_and_slide(self.motion, Vector2::zero(), false, 4 as i64, 0.785398 as f64, true);
    }

}