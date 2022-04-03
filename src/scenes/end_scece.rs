use macroquad::prelude::*;

use super::{
    game_scene::format_time,
    menu_scene::{Button, MenuAction, MenuScene},
};
use crate::{
    assets::Assets,
    scene::{Scene, SceneAction},
};

pub struct EndScene {
    button: Button,
    time: f32,
    words: String,
}

impl EndScene {
    pub fn new(time: f32) -> Box<Self> {
        show_mouse(true);
        set_cursor_grab(false);
        set_default_camera();

        let words = if time < 30.0 {
            "Ouch...".to_string()
        } else if time < 60.0 {
            "Float like a rock".to_string()
        } else if time < 90.0 {
            "Eh... Good enough".to_string()
        } else if time < 105.0 {
            "Great work".to_string()
        } else if time < 120.0 {
            "Top tier".to_string()
        } else {
            "Thanks for bothering".to_string()
        };

        Box::new(Self {
            button: Button {
                rect: Rect::new(25.0, 500.0, 350.0, 100.0),
                action: MenuAction::Return,
            },
            time,
            words,
        })
    }
}

impl Scene for EndScene {
    fn handle_input(&mut self) {}

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
            MenuAction::Return => SceneAction::Replace(MenuScene::new()),
            _ => SceneAction::Continue,
        }
    }

    fn render(&self, assets: &mut Assets) {
        clear_background(Color::from_rgba(52, 62, 65, 255));
        draw_rectangle(0.0, 75.0, screen_width(), 100.0, Color::from_rgba(196, 84, 91, 255));
        draw_text_ex(
            &format_time(self.time),
            50.0,
            150.0,
            TextParams {
                font: assets.font,
                font_size: 72,
                ..Default::default()
            },
        );
        draw_text_ex(
            &self.words,
            25.0,
            300.0,
            TextParams {
                font: assets.font,
                font_size: 32,
                ..Default::default()
            },
        );
        draw_rectangle(
            self.button.rect.x,
            self.button.rect.y,
            self.button.rect.w,
            self.button.rect.h,
            GRAY,
        );
        draw_text_ex(
            "Return to Menu",
            self.button.rect.x + 35.0,
            self.button.rect.y + 61.0,
            TextParams {
                font: assets.font,
                font_size: 40,
                ..Default::default()
            },
        );
    }
}
