// use macroquad::prelude::{load_texture, Texture2D};

pub struct Assets {
    // pub texture1: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        Self {
            // texture1: load_texture("assets/not_found.png").await.unwrap(),
        }
    }
}

// async fn get_texture(name: &str) -> Texture2D {
//     match load_texture(&format!("assets/{}.png", name)).await {
//         Ok(texture) => texture,
//         Err(_) => load_texture("assets/not_found.png")
//             .await
//             .expect("not_found texture is missing"),
//     }
// }
