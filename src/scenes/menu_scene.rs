use macroquad::prelude::*;

use super::game_scene::GameScene;
use crate::{
    assets::Assets,
    cursor::Cursor,
    level_gen::Difficulty,
    scene::{Scene, SceneAction},
};

#[derive(Clone, Copy)]
pub enum MenuAction {
    StartGame,
    StartGameHard,
    Return,
    None,
}

pub struct Button {
    pub rect: Rect,
    pub action: MenuAction,
}

pub struct MenuScene {
    cursor: Cursor,
    button: Button,
    button_2: Button,
}

impl MenuScene {
    pub fn new() -> Box<Self> {
        set_cursor_grab(false);
        set_default_camera();
        Box::new(Self {
            cursor: Cursor::new(),
            button: Button {
                rect: Rect::new(-100.0, 200.0, 350.0, 100.0),
                action: MenuAction::StartGame,
            },
            button_2: Button {
                rect: Rect::new(-100.0, 350.0, 350.0, 100.0),
                action: MenuAction::StartGameHard,
            },
        })
    }
}

impl Scene for MenuScene {
    fn handle_input(&mut self) {
        // update_inputs(&mut self.inputs, &self.bindings);
    }

    fn update(&mut self, _elapsed: f32) -> SceneAction {
        self.cursor.basic_update();
        let mut action = MenuAction::None;
        let mut mouse_pos = Vec2::default();
        (mouse_pos.x, mouse_pos.y) = mouse_position();
        self.button.rect.x = if self.button.rect.contains(mouse_pos) {
            -50.0
        } else {
            -100.0
        };
        self.button_2.rect.x = if self.button_2.rect.contains(mouse_pos) {
            -50.0
        } else {
            -100.0
        };
        if is_mouse_button_pressed(MouseButton::Left) {
            if self.button.rect.contains(mouse_pos) {
                action = self.button.action;
            }
            if self.button_2.rect.contains(mouse_pos) {
                action = self.button_2.action;
            }
        }
        match action {
            MenuAction::StartGame => SceneAction::Replace(GameScene::new(Difficulty::Normal)),
            MenuAction::StartGameHard => SceneAction::Replace(GameScene::new(Difficulty::Hard)),
            _ => SceneAction::Continue,
        }
    }

    fn render(&self, assets: &mut Assets) {
        draw_texture(assets.menu_bg, 0.0, 0.0, WHITE);
        draw_texture(assets.logo, 39.0, 20.0, WHITE);
        draw_texture(assets.menu_button_n, self.button.rect.x, self.button.rect.y, WHITE);
        draw_texture(assets.menu_button_h, self.button_2.rect.x, self.button_2.rect.y, WHITE);
        self.cursor.draw();
    }
}
