use macroquad::{
    prelude::*,
    rand::{gen_range, rand},
};

use crate::{
    bird::Bird,
    fling::{FlingKind, FlingThing},
    lightning::{Lightning, LIGHTING_CLOUD_DIM},
    player::{Player, PLAYER_DIM},
    HDirection,
};

const TIME_BETWEEN_BIRDS: f32 = 3.0;
const TIME_BETWEEN_LIGHTNING: f32 = 6.0;

pub struct Generator {
    bird_timer: f32,
    lightning_timer: f32,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            bird_timer: TIME_BETWEEN_BIRDS,
            lightning_timer: TIME_BETWEEN_LIGHTNING,
        }
    }

    pub fn generate(
        &mut self,
        camera: &Camera2D,
        player: &Player,
        birds: &mut Vec<Bird>,
        lightning: &mut Option<Lightning>,
        elapsed: f32,
    ) {
        self.bird_timer -= elapsed;
        self.lightning_timer -= elapsed;

        if self.bird_timer < 0.0 {
            let y_pos = gen_range(player.position.y + 20.0, camera.target.y + screen_width() / 2.0);
            let move_dir = match rand() % 2 {
                0 => HDirection::Left,
                _ => HDirection::Right,
            };
            birds.push(Bird::spawn(y_pos, move_dir));
            self.bird_timer = TIME_BETWEEN_BIRDS + gen_range(-1.0, 1.0);
        }

        if self.lightning_timer < 0.0 && lightning.is_none() {
            let x_pos = player.position.x + (PLAYER_DIM.0 - LIGHTING_CLOUD_DIM.0) / 2.0;
            let y_pos = camera.target.y - screen_height() / 2.0 - 100.0;
            *lightning = Some(Lightning::new(vec2(x_pos, y_pos)));
            self.lightning_timer = TIME_BETWEEN_LIGHTNING + gen_range(-2.0, 2.0);
        }
    }
}

pub fn generate_fling_things(ground_position: f32) -> Vec<FlingThing> {
    let mut things = Vec::new();

    let mut y_pos = 50.0;
    while y_pos < ground_position - 200.0 {
        let x_pos = gen_range(10.0, screen_width() - 10.0);
        y_pos += gen_range(50.0, 150.0);
        let kind = match rand() % 5 {
            0 => FlingKind::GoldCloud,
            _ => FlingKind::Cloud,
        };
        things.push(FlingThing::new(kind, vec2(x_pos, y_pos)));
    }

    things
}
