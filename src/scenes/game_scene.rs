use enum_map::{enum_map, Enum, EnumMap};
use macroquad::prelude::*;

use crate::{
    assets::Assets,
    scene::{Scene, SceneAction},
    update_inputs,
};

#[derive(Enum, Clone, Copy, Debug)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
}

pub struct GameScene {
    inputs: EnumMap<Input, bool>,
    bindings: EnumMap<Input, Vec<KeyCode>>,
}

impl GameScene {
    pub fn new() -> Self {
        let bindings = enum_map! {
            Input::Up => vec![KeyCode::W, KeyCode::Up],
            Input::Down => vec![KeyCode::S, KeyCode::Down],
            Input::Left => vec![KeyCode::A, KeyCode::Left],
            Input::Right => vec![KeyCode::D, KeyCode::Right],
        };
        Self {
            inputs: EnumMap::default(),
            bindings,
        }
    }
}

impl Scene for GameScene {
    fn handle_input(&mut self) {
        update_inputs(&mut self.inputs, &self.bindings);
    }

    fn update(&mut self, _elapsed: f32) -> SceneAction {
        if self.inputs[Input::Up] {
            SceneAction::Pop
        } else {
            SceneAction::Continue
        }
    }

    fn render(&self, assets: &mut Assets) {
        draw_circle(0.0, 0.0, 50.0, Color::from_rgba(255, 0, 0, 255));
        draw_texture(assets.texture1, 100.0, 100.0, Color::from_rgba(255, 255, 255, 255));
    }
}
