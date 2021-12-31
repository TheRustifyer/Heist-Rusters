use gdnative::prelude::*;
use gdrusthelper as gdrust;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[derive(Debug)]
pub struct Character;

#[gdnative::methods]
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
        owner.add_child(sprite, true);
        owner.set_position(Vector2::new(200.0, 200.0));
        
        for child in owner.get_children().into_iter() {
            godot_print!("Node: {:?}", &child);
        }
    }
}