use macroquad::prelude::Texture2D;

pub struct Assets {
    pub player: Texture2D,
    pub player_boost: Texture2D,
    pub background: Texture2D,
    pub ground: Texture2D,

    pub meter_full: Texture2D,
    pub meter_2: Texture2D,
    pub meter_1: Texture2D,
    pub meter_empty: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        Self {
            player: Texture2D::from_file_with_format(include_bytes!("../assets/player.png"), None),
            player_boost: Texture2D::from_file_with_format(include_bytes!("../assets/player_boost.png"), None),
            background: Texture2D::from_file_with_format(include_bytes!("../assets/background.png"), None),
            ground: Texture2D::from_file_with_format(include_bytes!("../assets/ground.png"), None),

            meter_full: Texture2D::from_file_with_format(include_bytes!("../assets/meter_full.png"), None),
            meter_2: Texture2D::from_file_with_format(include_bytes!("../assets/meter_2.png"), None),
            meter_1: Texture2D::from_file_with_format(include_bytes!("../assets/meter_1.png"), None),
            meter_empty: Texture2D::from_file_with_format(include_bytes!("../assets/meter_empty.png"), None),
        }
    }
}
