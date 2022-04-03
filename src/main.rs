mod assets;
mod bird;
mod cursor;
mod fling;
mod level_gen;
mod lightning;
mod player;
mod scene;
mod scenes;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use assets::Assets;
use enum_map::EnumMap;
use macroquad::{prelude::*, rand::srand};
use scene::SceneManager;
use scenes::menu_scene::MenuScene;
use serde::Deserialize;

pub enum HDirection {
    Left,
    Right,
}

#[derive(Deserialize)]
struct Config {}

fn window_conf() -> Conf {
    Conf {
        window_title: "Delirious Descent".to_owned(),
        window_width: 400,
        window_height: 640,
        window_resizable: false,
        high_dpi: false,
        icon: None,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), String> {
    let mut s = DefaultHasher::new();
    instant::Instant::now().hash(&mut s);
    let seed = s.finish();
    srand(seed);

    let mut scene_manager = SceneManager::new(MenuScene::new());
    let mut assets = Assets::load().await;
    show_mouse(false);

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
