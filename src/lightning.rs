use enum_map::EnumMap;
use macroquad::prelude::*;

use crate::{assets::Assets, scenes::game_scene::Sound};

pub const LIGHTING_CLOUD_DIM: (f32, f32) = (192.0, 96.0);
const LIGHTNING_SPEED: f32 = 120.0;
const LIGHTNING_TIMER: f32 = 5.0;
const BOLT_TIMER: f32 = 0.4;
const BOLT_WIDTH: f32 = 50.0;

#[derive(Debug)]
enum State {
    Appearing,
    Waiting,
    Striking { bolt_hitbox: Rect },
    Destroyed,
}

pub struct Lightning {
    state: State,
    position: Vec2,
    timer: f32,
    cloud_hitbox: Rect,
}

impl Lightning {
    pub fn new(position: Vec2) -> Lightning {
        Self {
            state: State::Appearing,
            position,
            timer: LIGHTNING_TIMER,
            cloud_hitbox: Rect::new(
                position.x,
                position.y,
                LIGHTING_CLOUD_DIM.0 - 10.0,
                LIGHTING_CLOUD_DIM.1 - 10.0,
            ),
        }
    }

    pub fn update(&mut self, camera: &Camera2D, sounds: &mut EnumMap<Sound, bool>, elapsed: f32) {
        match self.state {
            State::Appearing => {
                if self.position.y < camera.target.y - screen_height() / 2.0 {
                    self.position.y += LIGHTNING_SPEED * elapsed;
                } else {
                    self.state = State::Waiting;
                }
            }
            State::Waiting => {
                if self.timer < 0.0 {
                    sounds[Sound::Lightning] = true;
                    self.state = State::Striking {
                        bolt_hitbox: Rect::new(
                            self.position.x + (LIGHTING_CLOUD_DIM.0 - BOLT_WIDTH) / 2.0,
                            self.position.y,
                            BOLT_WIDTH,
                            screen_height() * 2.0,
                        ),
                    };
                    self.timer = BOLT_TIMER;
                }
            }
            State::Striking { .. } => {
                if self.timer < 0.0 {
                    self.state = State::Destroyed;
                    self.cloud_hitbox = Rect::default();
                }
            }
            State::Destroyed => {}
        }
        self.update_hitbox();
        self.timer -= elapsed;
    }

    pub fn draw(&self, assets: &Assets) {
        if matches!(self.state, State::Destroyed) {
            return;
        }
        if let State::Striking { bolt_hitbox } = self.state {
            let texture = match (self.timer * 10.0) as usize % 3 {
                0 => assets.lightning_1,
                1 => assets.lightning_2,
                _ => assets.lightning_3,
            };
            draw_texture(texture, bolt_hitbox.x, bolt_hitbox.y, WHITE);
            // draw_rectangle(
            //     bolt_hitbox.x,
            //     bolt_hitbox.y,
            //     bolt_hitbox.w,
            //     bolt_hitbox.h,
            //     Color::from_rgba(255, 0, 0, 128),
            // );
        }
        // draw_rectangle(
        //     self.cloud_hitbox.x,
        //     self.cloud_hitbox.y,
        //     self.cloud_hitbox.w,
        //     self.cloud_hitbox.h,
        //     Color::from_rgba(255, 0, 0, 128),
        // );
        draw_texture(assets.thunder_cloud, self.position.x, self.position.y, WHITE);
    }

    pub fn should_destroy(&self) -> bool {
        matches!(self.state, State::Destroyed)
    }

    fn update_hitbox(&mut self) {
        self.cloud_hitbox.x = self.position.x + (LIGHTING_CLOUD_DIM.0 - self.cloud_hitbox.w) / 2.0;
        self.cloud_hitbox.y = self.position.y + (LIGHTING_CLOUD_DIM.1 - self.cloud_hitbox.h) / 2.0;
    }

    pub fn collides_with(&self, other: &Rect) -> bool {
        if self.cloud_hitbox.overlaps(other) {
            return true;
        }
        if let State::Striking { bolt_hitbox } = self.state {
            if bolt_hitbox.overlaps(other) {
                return true;
            }
        }
        false
    }
}
