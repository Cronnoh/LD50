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
        let mut thing = Self {
            kind,
            position,
            velocity: Vec2::default(),
            state: State::Normal,
            hitbox: Rect::new(position.x, position.y, 48.0, 22.0),
        };
        thing.update_hitbox();
        thing
    }

    pub fn fling(&mut self, velocity: Vec2) {
        if !self.flung() && (velocity.x.abs() > 50.0 || velocity.y.abs() > 50.0) {
            self.velocity = velocity;
            self.velocity.x = f32::min(self.velocity.x, 250.0);
            self.velocity.x = f32::max(self.velocity.x, -250.0);
            self.velocity.y = f32::min(self.velocity.y, 250.0);
            self.velocity.y = f32::max(self.velocity.y, -250.0);
            self.state = State::Flung;
        }
    }

    pub fn update(&mut self, elapsed: f32) {
        if let State::Flung = self.state {
            self.position += self.velocity * elapsed;
            self.update_hitbox();
        }
    }

    pub fn draw(&self, assets: &Assets) {
        if matches!(self.state, State::Destroyed) {
            return;
        }
        let texture = match self.kind {
            FlingKind::Cloud => assets.cloud,
            FlingKind::GoldCloud => assets.gold_cloud,
        };
        draw_texture(texture, self.position.x, self.position.y, WHITE);
        // draw_rectangle(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, Color::from_rgba(255, 0, 0, 128));
    }

    fn update_hitbox(&mut self) {
        self.hitbox.x = self.position.x + (53.0 - self.hitbox.w) / 2.0;
        self.hitbox.y = self.position.y + (25.0 - self.hitbox.h) / 2.0;
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
