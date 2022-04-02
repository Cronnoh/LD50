use enum_map::EnumMap;
use macroquad::prelude::*;

use crate::{assets::Assets, game_scene::Input};
const HORIZONTAL_SPEED: f32 = 50.0;
const BOOSTER_SPEED: f32 = 500.0;
const BOOSTER_TIME: f32 = 0.25;

enum State {
    Normal,
    Booster { dir: HDirection, timer: f32 },
    Landed,
}

enum HDirection {
    Left,
    Right,
}

pub struct Player {
    state: State,
    pub position: Vec2,
    velocity: Vec2,
    balloons: usize,
    hitbox: Rect,
}

impl Player {
    pub fn new(starting_position: Vec2) -> Self {
        let hitbox = Rect::new(starting_position.x + 20.0, starting_position.y + 20.0, 40.0, 40.0);
        Self {
            state: State::Normal,
            position: starting_position,
            velocity: Vec2::default(),
            balloons: 3,
            hitbox,
        }
    }

    pub fn update(&mut self, inputs: &EnumMap<Input, bool>, elapsed: f32) {
        match (&self.state, inputs[Input::BoostLeft], inputs[Input::BoostRight]) {
            (State::Booster { .. }, ..) => {}
            (State::Landed, _, _) => {}
            (_, true, false) => {
                self.state = State::Booster {
                    dir: HDirection::Left,
                    timer: 0.0,
                }
            }
            (_, false, true) => {
                self.state = State::Booster {
                    dir: HDirection::Right,
                    timer: 0.0,
                }
            }
            _ => {}
        }

        match self.state {
            State::Normal => self.normal_update(inputs, elapsed),
            State::Booster { .. } => self.booster_update(inputs, elapsed),
            State::Landed => self.velocity = Vec2::new(0.0, 0.0),
        }

        self.position.x += self.velocity.x * elapsed;
        self.position.y += self.velocity.y * elapsed;
        self.update_hitbox();
    }

    fn normal_update(&mut self, inputs: &EnumMap<Input, bool>, _elapsed: f32) {
        self.velocity.y = match self.balloons {
            1 => 100.0,
            2 => 50.0,
            3 => 20.0,
            _ => 0.0,
        };
        if inputs[Input::Down] {
            self.velocity.y += 200.0;
        }
        match (inputs[Input::Left], inputs[Input::Right]) {
            (false, true) => self.velocity.x = HORIZONTAL_SPEED,
            (true, false) => self.velocity.x = -HORIZONTAL_SPEED,
            _ => self.velocity.x = 0.0,
        }
    }

    fn booster_update(&mut self, _inputs: &EnumMap<Input, bool>, elapsed: f32) {
        if let State::Booster {
            ref mut dir,
            ref mut timer,
        } = self.state
        {
            match dir {
                HDirection::Left => self.velocity.x = -BOOSTER_SPEED,
                HDirection::Right => self.velocity.x = BOOSTER_SPEED,
            }
            *timer += elapsed;
            self.velocity.y = 0.0;
            if *timer > BOOSTER_TIME {
                self.state = State::Normal;
            }
        }
    }

    pub fn draw(&self, assets: &Assets) {
        let texture = match self.state {
            State::Normal => assets.player,
            State::Booster { .. } => assets.player_boost,
            State::Landed => assets.player,
        };
        draw_texture(texture, self.position.x, self.position.y, WHITE);
        draw_rectangle(
            self.hitbox.x,
            self.hitbox.y,
            self.hitbox.w,
            self.hitbox.h,
            Color::from_rgba(0, 255, 0, 128),
        );
    }

    fn update_hitbox(&mut self) {
        self.hitbox.x = self.position.x + 25.0 - self.hitbox.w / 2.0;
        self.hitbox.y = self.position.y + 25.0 - self.hitbox.h / 2.0;
    }

    pub fn land(&mut self) {
        self.state = State::Landed;
    }
}
