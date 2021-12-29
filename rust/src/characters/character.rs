use gdnative::prelude::*;

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
        // Setting up the godot-rust project, but "salúdame siempre"
        godot_print!("Hello, Cacharelo!");
        
        // Remembering :)
        let sprite = Sprite::new();
        sprite.set_name("sprite_child");
        let sprite2 = Sprite::new();
        sprite2.set_name("sprite_child2");
        owner.add_child(sprite, true);
        owner.add_child(sprite2, true);
        
        for child in owner.get_children().into_iter() {
            godot_print!("Node: {:?}", &child);
        }
    }

}