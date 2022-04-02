use enum_map::{enum_map, Enum, EnumMap};
use macroquad::prelude::*;

use crate::{
    assets::Assets,
    player::Player,
    scene::{Scene, SceneAction},
    update_inputs,
};

#[derive(Enum, Clone, Copy, Debug)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    BoostLeft,
    BoostRight,
}

pub struct GameScene {
    player: Player,

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
            Input::BoostLeft => vec![KeyCode::Q, KeyCode::RightControl],
            Input::BoostRight => vec![KeyCode::E, KeyCode::Kp0],
        };
        Self {
            player: Player::new(Vec2::default()),

            inputs: EnumMap::default(),
            bindings,
        }
    }
}

impl Scene for GameScene {
    fn handle_input(&mut self) {
        update_inputs(&mut self.inputs, &self.bindings);
    }

    fn update(&mut self, elapsed: f32) -> SceneAction {
        self.player.update(&self.inputs, elapsed);

        SceneAction::Continue
    }

    fn render(&self, assets: &mut Assets) {
        self.player.draw(assets);
    }
}
