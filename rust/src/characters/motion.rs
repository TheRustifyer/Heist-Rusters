use gdnative::prelude::*;

use crate::utils::game_constants::in_game_constant::{
    CharacterConfiguration,
    CHARACTER_CONFIGURATION,
};

/// TODO Provide an extensive documentation for the implementation details
/// behind this trait, that it's just a part of a series of traits of player (character) motion
pub trait Motion2DWithMouse {
    fn move_character(
        self: &mut Self,
        owner: &KinematicBody2D,
        input_v: Option<&Input>,
        motion: &mut Vector2,
        player_config: CharacterConfiguration
    ) {
        // If there's Some(input) value inside self.input, unwrap it
        if let Some(input) = input_v{ 
            // Control the vertical motion
            if Input::is_action_pressed(input, "move_up") 
                && !Input::is_action_pressed(input, "move_down")  {
                motion.y -= player_config.move_speed;
            }
            else if Input::is_action_pressed(input, "move_down") 
                && !Input::is_action_pressed(input, "move_up")  {
                motion.y += player_config.move_speed;
            }
            else {
                motion.y = 0.0;
            }

            // Control the horizontal motion
            if Input::is_action_pressed(input, "move_left") 
                && !Input::is_action_pressed(input, "move_right")  {
                motion.x -= player_config.move_speed;
            }
            else if Input::is_action_pressed(input, "move_right") 
                && !Input::is_action_pressed(input, "move_left")  {
                motion.x += player_config.move_speed;
            }
            else {
                motion.x = 0.0;
            }   
        }
    }
}