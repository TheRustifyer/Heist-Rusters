pub mod in_game_constant {
    use gdrusthelper::{
        gdconfig::CharacterConfiguration,
        gdkeybinder::MotionKeybindings
    };

    pub const MOVE_SPEED: f32 = 10.0;
    pub const MAX_SPEED: f32 = 50.0;

    pub const UP: &'static str = "move_up";
    pub const DOWN: &'static str = "move_down";
    pub const LEFT: &'static str = "move_left";
    pub const RIGHT: &'static str = "move_right";

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
            friction: 0.0
        };
}

