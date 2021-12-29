use gdnative::prelude::*;

// Game crates
pub mod characters;

use characters::character::Character;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Character>();
} 

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);