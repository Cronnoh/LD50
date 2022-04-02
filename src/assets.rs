use macroquad::prelude::{load_texture, Texture2D};

pub struct Assets {
    pub player: Texture2D,
    pub player_boost: Texture2D,
    pub background: Texture2D,
    pub ground: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        Self {
            player: load_texture("assets/player.png").await.unwrap(),
            player_boost: load_texture("assets/player_boost.png").await.unwrap(),
            background: load_texture("assets/background.png").await.unwrap(),
            ground: load_texture("assets/ground.png").await.unwrap(),
        }
    }
}
