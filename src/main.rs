mod assets;
mod player;
mod scene;
mod scenes;

use assets::Assets;
use enum_map::EnumMap;
use macroquad::prelude::*;
use scene::SceneManager;
use scenes::game_scene;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {}

fn window_conf() -> Conf {
    Conf {
        window_title: "idk".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        icon: None,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), String> {
    let _config: Config = ron::from_str(include_str!("../config/config.ron")).map_err(|e| e.to_string())?;
    let mut scene_manager = SceneManager::new(Box::new(game_scene::GameScene::new()));
    let mut assets = Assets::load().await;

    loop {
        let elapsed = get_frame_time();
        scene_manager.update(&mut assets, elapsed);
        next_frame().await
    }
}

fn update_inputs<T>(inputs: &mut EnumMap<T, bool>, bindings: &EnumMap<T, Vec<KeyCode>>)
where
    T: enum_map::EnumArray<bool> + enum_map::EnumArray<Vec<KeyCode>> + Copy,
{
    for (input, buttons) in bindings {
        inputs[input] = false;
        for button in buttons {
            if is_key_down(*button) {
                inputs[input] = true;
                break;
            }
        }
    }
}
