use macroquad::prelude::{load_ttf_font_from_bytes, Font, Texture2D};

pub struct Assets {
    pub player: Texture2D,
    pub player_2: Texture2D,
    pub player_1: Texture2D,
    pub player_0: Texture2D,
    pub flame: Texture2D,
    pub background: Texture2D,
    pub ground: Texture2D,

    pub meter_full: Texture2D,
    pub meter_2: Texture2D,
    pub meter_1: Texture2D,
    pub meter_empty: Texture2D,

    pub cloud: Texture2D,
    pub gold_cloud: Texture2D,

    pub bird_1: Texture2D,
    pub bird_2: Texture2D,

    pub thunder_cloud: Texture2D,
    pub lightning_1: Texture2D,
    pub lightning_2: Texture2D,
    pub lightning_3: Texture2D,

    pub logo: Texture2D,
    pub menu_button_n: Texture2D,
    pub menu_button_h: Texture2D,
    pub menu_bg: Texture2D,

    pub font: Font,
}

impl Assets {
    pub async fn load() -> Self {
        Self {
            player: Texture2D::from_file_with_format(include_bytes!("../assets/player.png"), None),
            player_2: Texture2D::from_file_with_format(include_bytes!("../assets/player_2.png"), None),
            player_1: Texture2D::from_file_with_format(include_bytes!("../assets/player_1.png"), None),
            player_0: Texture2D::from_file_with_format(include_bytes!("../assets/player_0.png"), None),
            flame: Texture2D::from_file_with_format(include_bytes!("../assets/flame.png"), None),
            background: Texture2D::from_file_with_format(include_bytes!("../assets/background.png"), None),
            ground: Texture2D::from_file_with_format(include_bytes!("../assets/ground.png"), None),

            meter_full: Texture2D::from_file_with_format(include_bytes!("../assets/meter_full.png"), None),
            meter_2: Texture2D::from_file_with_format(include_bytes!("../assets/meter_2.png"), None),
            meter_1: Texture2D::from_file_with_format(include_bytes!("../assets/meter_1.png"), None),
            meter_empty: Texture2D::from_file_with_format(include_bytes!("../assets/meter_empty.png"), None),

            cloud: Texture2D::from_file_with_format(include_bytes!("../assets/cloud.png"), None),
            gold_cloud: Texture2D::from_file_with_format(include_bytes!("../assets/gold_cloud.png"), None),

            bird_1: Texture2D::from_file_with_format(include_bytes!("../assets/bird_1.png"), None),
            bird_2: Texture2D::from_file_with_format(include_bytes!("../assets/bird_2.png"), None),

            thunder_cloud: Texture2D::from_file_with_format(include_bytes!("../assets/thunder_cloud.png"), None),
            lightning_1: Texture2D::from_file_with_format(include_bytes!("../assets/lightning_1.png"), None),
            lightning_2: Texture2D::from_file_with_format(include_bytes!("../assets/lightning_2.png"), None),
            lightning_3: Texture2D::from_file_with_format(include_bytes!("../assets/lightning_3.png"), None),

            logo: Texture2D::from_file_with_format(include_bytes!("../assets/logo.png"), None),
            menu_button_n: Texture2D::from_file_with_format(include_bytes!("../assets/menu_button_n.png"), None),
            menu_button_h: Texture2D::from_file_with_format(include_bytes!("../assets/menu_button_h.png"), None),
            menu_bg: Texture2D::from_file_with_format(include_bytes!("../assets/menu_bg.png"), None),

            font: load_ttf_font_from_bytes(include_bytes!("../assets/UbuntuMono-B.ttf")).unwrap(),
        }
    }
}
