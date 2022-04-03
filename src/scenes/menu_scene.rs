use macroquad::prelude::*;

use super::game_scene::GameScene;
use crate::{
    assets::Assets,
    scene::{Scene, SceneAction},
};

#[derive(Clone, Copy)]
enum MenuAction {
    StartGame,
    None,
}

struct Button {
    rect: Rect,
    action: MenuAction,
}

pub struct MenuScene {
    button: Button,
}

impl MenuScene {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            button: Button {
                rect: Rect::new(0.0, 200.0, 350.0, 100.0),
                action: MenuAction::StartGame,
            },
        })
    }
}

impl Scene for MenuScene {
    fn handle_input(&mut self) {
        // update_inputs(&mut self.inputs, &self.bindings);
    }

    fn update(&mut self, _elapsed: f32) -> SceneAction {
        let mut action = MenuAction::None;
        if is_mouse_button_pressed(MouseButton::Left) {
            let mut mouse_pos = Vec2::default();
            (mouse_pos.x, mouse_pos.y) = mouse_position();
            if self.button.rect.contains(mouse_pos) {
                action = self.button.action;
            }
        }
        match action {
            MenuAction::StartGame => SceneAction::Replace(GameScene::new()),
            MenuAction::None => SceneAction::Continue,
        }
    }

    fn render(&self, _assets: &mut Assets) {
        draw_rectangle(
            self.button.rect.x,
            self.button.rect.y,
            self.button.rect.w,
            self.button.rect.h,
            BLUE,
        );
    }
}
