use enum_map::{enum_map, Enum, EnumMap};
use macroquad::prelude::*;

use crate::{
    assets::Assets,
    bird::Bird,
    cursor::Cursor,
    fling::{FlingKind, FlingThing},
    lightning::Lightning,
    player::Player,
    scene::{Scene, SceneAction},
    update_inputs, HDirection,
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
    cursor: Cursor,
    fling_things: Vec<FlingThing>,
    birds: Vec<Bird>,
    lightning: Option<Lightning>,
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

        let fling_things = vec![
            FlingThing::new(FlingKind::Cloud, vec2(300.0, 300.0)),
            FlingThing::new(FlingKind::Cloud, vec2(200.0, 500.0)),
            FlingThing::new(FlingKind::Cloud, vec2(220.0, 520.0)),
            FlingThing::new(FlingKind::GoldCloud, vec2(341.0, 614.0)),
            FlingThing::new(FlingKind::GoldCloud, vec2(152.0, 199.0)),
            FlingThing::new(FlingKind::Cloud, vec2(150.0, 700.0)),
        ];

        let birds = vec![Bird::spawn(300.0, HDirection::Left)];

        let scene = Self {
            player: Player::new(Vec2::default()),
            cursor: Cursor::new(),
            fling_things,
            birds,
            lightning: None,
            time: 0.0,
            ground_position: 1000.0,

            camera,
            inputs: EnumMap::default(),
            bindings,
        };
        set_camera(&scene.camera);
        scene
    }

    fn check_collisions(&mut self) {
        for thing in self.fling_things.iter_mut() {
            if thing.hitbox.overlaps(&self.player.hitbox) {
                self.player.thing_collision(&thing);
                thing.collision();
            }
        }
        for bird in self.birds.iter_mut() {
            if bird.hitbox.overlaps(&self.player.hitbox) {
                self.player.bird_collision();
                bird.collision();
            }
        }
        for thing in self.fling_things.iter_mut() {
            for bird in self.birds.iter_mut() {
                if thing.flung() && bird.hitbox.overlaps(&thing.hitbox) {
                    bird.collision();
                    thing.collision();
                }
            }
        }
    }
}

impl Scene for GameScene {
    fn handle_input(&mut self) {
        update_inputs(&mut self.inputs, &self.bindings);
    }

    fn update(&mut self, elapsed: f32) -> SceneAction {
        self.player.update(&self.inputs, elapsed);
        self.cursor.update(&self.camera, &mut self.fling_things);
        for thing in self.fling_things.iter_mut() {
            thing.update(elapsed);
        }
        for bird in self.birds.iter_mut() {
            bird.update(&self.player, elapsed);
        }

        if self.ground_position + 60.0 > self.camera.target.y + screen_height() / 2.0 {
            self.camera.target.y = self.player.position.y + screen_height() / 3.0;
            set_camera(&self.camera);
        }
        if self.player.position.y > 500.0 && self.lightning.is_none() {
            self.lightning = Some(Lightning::new(vec2(150.0, self.player.position.y - 200.0)));
        }

        if let Some(ref mut lightning) = self.lightning {
            lightning.update(&self.camera, elapsed);
        }

        self.check_collisions();

        if self.player.position.y + 50.0 >= self.ground_position {
            self.player.land();
        } else {
            self.time += elapsed;
        }
        SceneAction::Continue
    }

    fn render(&self, assets: &mut Assets) {
        draw_background(&self.camera, assets);
        self.player.draw(assets);
        if let Some(ref lightning) = self.lightning {
            lightning.draw(assets);
        }
        if self.ground_position < self.camera.target.y + screen_height() / 2.0 {
            draw_texture(
                assets.ground,
                0.0,
                self.ground_position,
                Color::from_rgba(255, 255, 255, 255),
            );
        }
        for thing in self.fling_things.iter() {
            thing.draw(assets);
        }
        for bird in self.birds.iter() {
            bird.draw(assets);
        }

        let top_bar_pos = self.camera.screen_to_world(Vec2::new(0.0, 0.0));
        draw_rectangle(top_bar_pos.x, top_bar_pos.y, screen_width(), 40.0, BLACK);

        let text_pos = self.camera.screen_to_world(Vec2::new(screen_width() - 170.0, 30.0));
        draw_text(&format_time(self.time), text_pos.x, text_pos.y, 45.0, WHITE);
        let fuel_pos = self.camera.screen_to_world(Vec2::new(15.0, 30.0));
        draw_text(&self.player.fuel.to_string(), fuel_pos.x, fuel_pos.y, 45.0, WHITE);
        self.cursor.draw();
    }
}

fn draw_background(camera: &Camera2D, assets: &mut Assets) {
    // assumes assets.background is the same height as screen_height
    let y_pos = screen_height() * f32::trunc(camera.target.y / screen_height());
    draw_texture(assets.background, 0.0, y_pos, WHITE);
    let second_pos = if camera.target.y - y_pos > screen_height() / 2.0 {
        //draw above
        y_pos + screen_height()
    } else {
        // draw below
        y_pos - screen_height()
    };
    draw_texture(assets.background, 0.0, second_pos, WHITE);
}

pub fn format_time(seconds: f32) -> String {
    let minutes = f32::trunc(seconds / 60.0);
    let seconds = seconds - 60.0 * minutes;

    format!("{:>02}:{:>05.2}", minutes, seconds)
}
