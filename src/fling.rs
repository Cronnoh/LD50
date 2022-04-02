use macroquad::prelude::*;

use crate::assets::Assets;

pub enum FlingKind {
    Cloud,
}

pub struct FlingThing {
    pub kind: FlingKind,
    position: Vec2,
    velocity: Vec2,
    flung: bool,
    pub hitbox: Rect,
}

impl FlingThing {
    pub fn new(kind: FlingKind, position: Vec2) -> Self {
        Self {
            kind,
            position,
            velocity: Vec2::default(),
            flung: false,
            hitbox: Rect::new(position.x, position.y, 75.0, 50.0),
        }
    }

    pub fn fling(&mut self, velocity: Vec2) {
        if !self.flung {
            self.velocity = velocity;
            self.flung = true;
        }
    }

    pub fn update(&mut self, elapsed: f32) {
        if self.flung {
            self.position += self.velocity * elapsed;
            self.update_hitbox();
        }
    }

    pub fn draw(&self, _assets: &Assets) {
        draw_rectangle(self.position.x, self.position.y, 75.0, 50.0, GRAY);
    }

    fn update_hitbox(&mut self) {
        self.hitbox.x = self.position.x + 37.5 - self.hitbox.w / 2.0;
        self.hitbox.y = self.position.y + 25.0 - self.hitbox.h / 2.0;
    }
}
