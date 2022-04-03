use macroquad::prelude::*;

use crate::assets::Assets;

pub enum FlingKind {
    Cloud,
    GoldCloud,
}

enum State {
    Normal,
    Flung,
    Destroyed,
}

pub struct FlingThing {
    pub kind: FlingKind,
    pub position: Vec2,
    pub velocity: Vec2,
    state: State,
    pub hitbox: Rect,
}

impl FlingThing {
    pub fn new(kind: FlingKind, position: Vec2) -> Self {
        Self {
            kind,
            position,
            velocity: Vec2::default(),
            state: State::Normal,
            hitbox: Rect::new(position.x, position.y, 75.0, 50.0),
        }
    }

    pub fn fling(&mut self, velocity: Vec2) {
        if !self.flung() {
            self.velocity = velocity;
            self.state = State::Flung;
        }
    }

    pub fn update(&mut self, elapsed: f32) {
        match self.state {
            State::Flung => {
                self.position += self.velocity * elapsed;
                self.update_hitbox();
            }
            _ => {}
        }
    }

    pub fn draw(&self, _assets: &Assets) {
        if matches!(self.state, State::Destroyed) {
            return;
        }
        match self.kind {
            FlingKind::Cloud => draw_rectangle(self.position.x, self.position.y, 75.0, 50.0, GRAY),
            FlingKind::GoldCloud => draw_rectangle(self.position.x, self.position.y, 75.0, 50.0, GOLD),
        }
    }

    fn update_hitbox(&mut self) {
        self.hitbox.x = self.position.x + 37.5 - self.hitbox.w / 2.0;
        self.hitbox.y = self.position.y + 25.0 - self.hitbox.h / 2.0;
    }

    pub fn flung(&self) -> bool {
        matches!(self.state, State::Flung)
    }

    pub fn collision(&mut self) {
        self.state = State::Destroyed;
        self.hitbox = Rect::default();
    }

    pub fn should_destroy(&self) -> bool {
        matches!(self.state, State::Destroyed)
    }
}
