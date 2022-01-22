use gdnative::prelude::*;
use gdrusthelper as gdrust;

use gdrust::gdmotion::KeysMotionMouseDirection;

use crate::utils::constants::in_game_constant::{
    CHARACTER_CONFIGURATION,
    MOTION_KEYBINDINGS
};

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[derive(Debug)]
pub struct Character { }

impl KeysMotionMouseDirection for Character { }

#[gdnative::prelude::methods]
impl Character {
    pub fn new(_owner: &KinematicBody2D) -> Self { 
        Self { }
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
        
        // Just for debug the child nodes
        for child in owner.get_children().into_iter() {
            godot_print!("Node: {:?}", &child);
        }
    }

    #[export]
    fn _physics_process(&self, owner: &KinematicBody2D, _delta: f32) {
        let motion: Vector2 = self.move_character(
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