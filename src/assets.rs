use macroquad::prelude::Texture2D;

pub struct Assets {
    pub player: Texture2D,
    pub player_boost: Texture2D,
    pub background: Texture2D,
    pub ground: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        Self {
            player: Texture2D::from_file_with_format(include_bytes!("../assets/player.png"), None),
            player_boost: Texture2D::from_file_with_format(include_bytes!("../assets/player_boost.png"), None),
            background: Texture2D::from_file_with_format(include_bytes!("../assets/background.png"), None),
            ground: Texture2D::from_file_with_format(include_bytes!("../assets/ground.png"), None),
        }
    }
}
