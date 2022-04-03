use enum_map::EnumMap;
use macroquad::prelude::*;

use crate::{
    assets::Assets,
    fling::{FlingKind, FlingThing},
    scenes::game_scene::{Input, Sound},
    HDirection,
};

pub const PLAYER_DIM: (f32, f32) = (64.0, 64.0);
const HORIZONTAL_SPEED: f32 = 50.0;
const BOOSTER_SPEED: f32 = 500.0;
const BOOSTER_TIME: f32 = 0.25;

enum State {
    Normal,
    Booster { dir: HDirection, timer: f32 },
    Landed,
    Bounced { timer: f32 },
}

pub struct Player {
    state: State,
    pub position: Vec2,
    pub velocity: Vec2,
    balloons: usize,
    pub hitbox: Rect,
    pub fuel: usize,
    boost_cooldown: f32,
    invincible: f32,
}

impl Player {
    pub fn new(starting_position: Vec2) -> Self {
        let hitbox = Rect::new(starting_position.x + 8.0, starting_position.y + 8.0, 48.0, 48.0);
        Self {
            state: State::Normal,
            position: starting_position,
            velocity: Vec2::default(),
            balloons: 3,
            hitbox,
            fuel: 3,
            boost_cooldown: 0.0,
            invincible: 0.0,
        }
    }

    pub fn update(&mut self, inputs: &EnumMap<Input, bool>, sounds: &mut EnumMap<Sound, bool>, elapsed: f32) {
        if self.fuel > 0
            && self.boost_cooldown <= 0.0
            && !matches!(self.state, State::Booster { .. })
            && !matches!(self.state, State::Landed)
        {
            match (inputs[Input::BoostLeft], inputs[Input::BoostRight]) {
                (true, false) => {
                    sounds[Sound::Boost] = true;
                    self.fuel -= 1;
                    self.boost_cooldown = 0.25;
                    self.state = State::Booster {
                        dir: HDirection::Left,
                        timer: 0.0,
                    }
                }
                (false, true) => {
                    sounds[Sound::Boost] = true;
                    self.fuel -= 1;
                    self.boost_cooldown = 0.25;
                    self.state = State::Booster {
                        dir: HDirection::Right,
                        timer: 0.0,
                    }
                }
                _ => {}
            }
        } else {
            self.boost_cooldown -= elapsed;
        }

        match self.state {
            State::Normal => self.normal_update(inputs, elapsed),
            State::Booster { .. } => self.booster_update(inputs, elapsed),
            State::Landed => self.velocity = Vec2::new(0.0, 0.0),
            State::Bounced { ref mut timer } => {
                *timer -= elapsed;
                self.velocity.y += 1.0;
                if *timer < 0.0 {
                    self.state = State::Normal;
                }
            }
        }

        self.position.x += self.velocity.x * elapsed;
        self.position.x = f32::max(self.position.x, 0.0);
        self.position.x = f32::min(self.position.x, screen_width() - PLAYER_DIM.0);
        self.position.y += self.velocity.y * elapsed;
        self.invincible -= elapsed;
        self.update_hitbox();
    }

    fn normal_update(&mut self, inputs: &EnumMap<Input, bool>, _elapsed: f32) {
        self.velocity.y = match self.balloons {
            0 => 500.0,
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
        let texture = match self.balloons {
            0 => assets.player_0,
            1 => assets.player_1,
            2 => assets.player_2,
            _ => assets.player,
        };
        let color = if self.invincible > 0.0 { RED } else { WHITE };
        draw_texture(texture, self.position.x, self.position.y, color);
        // draw_rectangle(
        //     self.hitbox.x,
        //     self.hitbox.y,
        //     self.hitbox.w,
        //     self.hitbox.h,
        //     Color::from_rgba(255, 0, 0, 128),
        // );

        if let State::Booster { ref dir, .. } = self.state {
            let (flip, pos) = match dir {
                HDirection::Left => (true, vec2(self.position.x + 47.0, self.position.y + 40.0)),
                HDirection::Right => (false, vec2(self.position.x, self.position.y + 40.0)),
            };
            draw_texture_ex(
                assets.flame,
                pos.x,
                pos.y,
                WHITE,
                DrawTextureParams {
                    flip_x: flip,
                    ..Default::default()
                },
            )
        }
    }

    fn update_hitbox(&mut self) {
        self.hitbox.x = self.position.x + (PLAYER_DIM.0 - self.hitbox.w) / 2.0;
        self.hitbox.y = self.position.y + (PLAYER_DIM.1 - self.hitbox.h) / 2.0;
    }

    pub fn land(&mut self) {
        self.state = State::Landed;
    }

    pub fn take_damage(&mut self, sounds: &mut EnumMap<Sound, bool>) {
        if self.invincible <= 0.0 {
            sounds[Sound::Hit] = true;
            self.balloons = self.balloons.saturating_sub(1);
            self.invincible = 0.5;
        }
    }

    pub fn thing_collision(&mut self, thing: &FlingThing) {
        match thing.kind {
            FlingKind::Cloud => {
                if thing.flung() {
                    self.velocity += thing.velocity;
                    self.state = State::Bounced { timer: 0.5 };
                }
            }
            FlingKind::GoldCloud => {
                self.fuel = std::cmp::min(self.fuel + 1, 3);
            }
        }
    }

    pub fn bird_collision(&mut self, sounds: &mut EnumMap<Sound, bool>) {
        self.take_damage(sounds);
    }

    pub fn lightning_collision(&mut self, sounds: &mut EnumMap<Sound, bool>) {
        self.take_damage(sounds);
    }
}
