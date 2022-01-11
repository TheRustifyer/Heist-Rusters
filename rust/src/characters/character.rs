use gdnative::prelude::*;
use gdrusthelper as gdrust;

use crate::utils::game_constants::in_game_constant::{
    MOVE_SPEED
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
        owner.add_child(sprite, true);
        owner.set_position(Vector2::new(200.0, 200.0));
        
        for child in owner.get_children().into_iter() {
            godot_print!("Node: {:?}", &child);
        }
    }

    #[export]
    fn _physics_process(&self, owner: &KinematicBody2D, delta: f32) {
        
    }

    fn move_character(&self) {
        if Input::is_action_pressed(self.input.unwrap(), "move_up") 
            && !Input::is_action_pressed(self.input.unwrap(), "move_down")  {
            self.motion.y -= MOVE_SPEED;
        }
        if Input::is_action_pressed(self.input.unwrap(), "move_down") 
            && !Input::is_action_pressed(self.input.unwrap(), "move_up")  {
            self.motion.y += MOVE_SPEED;
        }

            

    }
}