use macroquad::{prelude::*, rand::gen_range};

use crate::{assets::Assets, player::Player, HDirection};
const BIRD_SPEED: f32 = 120.0;
const BIRD_PREDICTION_TIME: f32 = 5.0;
const BIRD_SIZE: f32 = 30.0;

#[derive(Debug)]
enum State {
    Arrival,
    Target,
    Flight,
}

pub struct Bird {
    state: State,
    move_dir: HDirection,
    position: Vec2,
    velocity: Vec2,
    target_pos: Vec2,
    pause_timer: f32,
    hitbox: Rect,
}

impl Bird {
    pub fn spawn(y_pos: f32, move_dir: HDirection) -> Self {
        let (x_pos, velocity) = match move_dir {
            HDirection::Left => (screen_width() + 25.0, vec2(-BIRD_SPEED, 0.0)),
            HDirection::Right => (-25.0, vec2(BIRD_SPEED, 0.0)),
        };

        Self {
            state: State::Arrival,
            move_dir,
            position: vec2(x_pos, y_pos),
            velocity,
            target_pos: Vec2::default(),
            pause_timer: 0.0,
            hitbox: Rect::new(x_pos, y_pos, BIRD_SIZE, BIRD_SIZE),
        }
    }

    pub fn update(&mut self, player: &Player, elapsed: f32) {
        match self.state {
            State::Arrival => {
                let in_h_pos = match self.move_dir {
                    HDirection::Left => self.position.x < screen_width() - BIRD_SIZE,
                    HDirection::Right => self.position.x > BIRD_SIZE,
                };
                let in_v_pos = player.position.y > self.position.y;
                match (in_h_pos, in_v_pos) {
                    (true, true) => self.enter_state(player, State::Target),
                    (true, false) => self.velocity = vec2(0.0, -BIRD_SPEED),
                    (false, true) => self.velocity.y = 0.0,
                    (false, false) => {}
                }
            }
            State::Target => {
                if self.position.y >= self.target_pos.y {
                    self.velocity.y = 0.0;
                }
                self.pause_timer -= elapsed;
                if self.pause_timer < 0.0 {
                    self.enter_state(player, State::Flight);
                }
            }
            State::Flight => {}
        }
        self.position += self.velocity * elapsed;
        self.update_hitbox();
    }

    pub fn draw(&self, _assets: &mut Assets) {
        draw_rectangle(self.hitbox.x, self.hitbox.y, BIRD_SIZE, BIRD_SIZE, BEIGE);
    }

    fn update_hitbox(&mut self) {
        self.hitbox.x = self.position.x + (BIRD_SIZE - self.hitbox.w) / 2.0;
        self.hitbox.y = self.position.y + (BIRD_SIZE - self.hitbox.h) / 2.0;
    }

    fn enter_state(&mut self, player: &Player, state: State) {
        match state {
            State::Arrival => {}
            State::Target => {
                self.target_player(player);
                self.velocity = vec2(0.0, BIRD_SPEED);
                self.pause_timer = BIRD_PREDICTION_TIME - (self.target_pos.x - self.position.x).abs() / BIRD_SPEED;
                self.pause_timer += gen_range(-0.25, 0.25);
            }
            State::Flight => match self.move_dir {
                HDirection::Left => self.velocity = vec2(-BIRD_SPEED, 0.0),
                HDirection::Right => self.velocity = vec2(BIRD_SPEED, 0.0),
            },
        }
        self.state = state;
    }

    fn target_player(&mut self, player: &Player) {
        // player's position in 2 sec
        self.target_pos = player.position + player.velocity * BIRD_PREDICTION_TIME;
    }
}
