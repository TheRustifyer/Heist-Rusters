pub mod in_game_constant {
    use gdrusthelper::{
        gdconfig::CharacterConfiguration,
        gdkeybinder::MotionKeybindings
    };

    const MOVE_SPEED: f32 = 500.0;
    const MAX_SPEED: f32 = 1000.0;

    const UP: &'static str = "move_up";
    const DOWN: &'static str = "move_down";
    const LEFT: &'static str = "move_left";
    const RIGHT: &'static str = "move_right";

    pub const MOTION_KEYBINDINGS: MotionKeybindings = 
        MotionKeybindings {
            up: UP, 
            down: DOWN, 
            left: LEFT, 
            right: RIGHT
        };

    pub const CHARACTER_CONFIGURATION: CharacterConfiguration = 
        CharacterConfiguration {
            move_speed: MOVE_SPEED,
            max_speed: MAX_SPEED,
            friction: 0.9
        };
}

