pub mod in_game_constant {
    pub const MOVE_SPEED: f32 = 10.0;
    pub const MAX_SPEED: f32 = 50.0;

    pub struct CharacterConfiguration {
        pub move_speed: f32,
        pub max_speed: f32
    }

    pub const CHARACTER_CONFIGURATION: CharacterConfiguration = 
        CharacterConfiguration {
            move_speed: MOVE_SPEED,
            max_speed: MAX_SPEED
        };
}

