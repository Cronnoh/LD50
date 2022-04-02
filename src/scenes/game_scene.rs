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
    time: f32,
    ground_position: f32,

    camera: Camera2D,
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

        let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));

        let scene = Self {
            player: Player::new(Vec2::default()),
            time: 0.0,
            ground_position: 1000.0,

            camera,
            inputs: EnumMap::default(),
            bindings,
        };
        set_camera(&scene.camera);
        scene
    }
}

impl Scene for GameScene {
    fn handle_input(&mut self) {
        update_inputs(&mut self.inputs, &self.bindings);
    }

    fn update(&mut self, elapsed: f32) -> SceneAction {
        self.player.update(&self.inputs, elapsed);
        if self.ground_position + 60.0 > self.camera.target.y + screen_height() / 2.0 {
            self.camera.target.y = self.player.position.y + screen_height() / 3.0;
            set_camera(&self.camera);
        }
        if self.player.position.y + 50.0 >= self.ground_position {
            self.player.land();
        } else {
            self.time += elapsed;
            println!("{}", self.camera.target);
        }
        SceneAction::Continue
    }

    fn render(&self, assets: &mut Assets) {
        draw_background(&self.camera, assets);
        self.player.draw(assets);
        if self.ground_position < self.camera.target.y + screen_height() / 2.0 {
            draw_texture(
                assets.ground,
                0.0,
                self.ground_position,
                Color::from_rgba(255, 255, 255, 255),
            );
        }
        let top_bar_pos = self.camera.screen_to_world(Vec2::new(0.0, 0.0));
        draw_rectangle(
            top_bar_pos.x,
            top_bar_pos.y,
            screen_width(),
            40.0,
            Color::from_rgba(0, 0, 0, 255),
        );

        let text_pos = self.camera.screen_to_world(Vec2::new(screen_width() - 170.0, 30.0));
        draw_text(
            &format_time(self.time),
            text_pos.x,
            text_pos.y,
            45.0,
            Color::from_rgba(255, 255, 255, 255),
        );
    }
}

fn draw_background(camera: &Camera2D, assets: &mut Assets) {
    // assumes assets.background is the same height as screen_height
    let y_pos = screen_height() * f32::trunc(camera.target.y / screen_height());
    draw_texture(assets.background, 0.0, y_pos, Color::from_rgba(255, 255, 255, 255));
    let second_pos = if camera.target.y - y_pos > screen_height() / 2.0 {
        //draw above
        y_pos + screen_height()
    } else {
        // draw below
        y_pos - screen_height()
    };
    draw_texture(assets.background, 0.0, second_pos, Color::from_rgba(255, 255, 255, 255));
}

pub fn format_time(seconds: f32) -> String {
    let minutes = f32::trunc(seconds / 60.0);
    let seconds = seconds - 60.0 * minutes;

    format!("{:>02}:{:>05.2}", minutes, seconds)
}
