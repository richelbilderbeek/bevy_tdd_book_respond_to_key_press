use bevy::prelude::*;

/// Player elements that cannot be put in a SpriteBundle
#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
}

#[cfg(test)]
pub fn create_default_player() -> Player {
    Player {
        velocity: Vec2::new(0.0, 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_player() {
        create_default_player();
    }
    #[test]
    fn test_initial_player_velocity() {
        assert_eq!(create_default_player().velocity, Vec2::new(0.0, 0.0));
    }
}
