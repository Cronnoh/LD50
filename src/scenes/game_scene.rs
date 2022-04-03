use enum_map::{enum_map, Enum, EnumMap};
use macroquad::prelude::*;

use crate::{
    assets::Assets,
    bird::Bird,
    cursor::Cursor,
    fling::FlingThing,
    level_gen::{self, Generator},
    lightning::Lightning,
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
    cursor: Cursor,
    generator: Generator,
    fling_things: Vec<FlingThing>,
    birds: Vec<Bird>,
    lightning: Option<Lightning>,
    time: f32,
    ground_position: f32,
    mouse_captured: bool,

    camera: Camera2D,
    inputs: EnumMap<Input, bool>,
    bindings: EnumMap<Input, Vec<KeyCode>>,
}

impl GameScene {
    pub fn new() -> Box<Self> {
        let bindings = enum_map! {
            Input::Up => vec![KeyCode::W, KeyCode::Up],
            Input::Down => vec![KeyCode::S, KeyCode::Down],
            Input::Left => vec![KeyCode::A, KeyCode::Left],
            Input::Right => vec![KeyCode::D, KeyCode::Right],
            Input::BoostLeft => vec![KeyCode::Q, KeyCode::RightControl],
            Input::BoostRight => vec![KeyCode::E, KeyCode::Kp0],
        };

        show_mouse(false);
        set_cursor_grab(true);

        let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
        let ground_position = 1000.0;

        let scene = Self {
            player: Player::new(vec2(screen_width() / 2.0, 10.0)),
            cursor: Cursor::new(),
            generator: Generator::new(),
            fling_things: level_gen::generate_fling_things(ground_position),
            birds: Vec::new(),
            lightning: None,
            time: 0.0,
            ground_position,
            mouse_captured: true,

            camera,
            inputs: EnumMap::default(),
            bindings,
        };
        set_camera(&scene.camera);
        Box::new(scene)
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
        if let Some(ref lightning) = self.lightning {
            if lightning.collides_with(&self.player.hitbox) {
                self.player.lightning_collision();
            }
        }
    }

    fn clean_up(&mut self) {
        let play_zone = Rect::new(
            -screen_width() / 2.0,
            self.camera.target.y - screen_height(),
            screen_width() * 2.0,
            screen_height() * 2.0,
        );

        let mut remove = Vec::new();
        for (i, bird) in self.birds.iter_mut().enumerate() {
            if !bird.hitbox.overlaps(&play_zone) {
                remove.push(i);
            }
        }
        for i in remove.iter().rev() {
            self.birds.swap_remove(*i);
        }

        if !self.cursor.has_selected() {
            let mut remove = Vec::new();
            for (i, thing) in self.fling_things.iter_mut().enumerate() {
                if !thing.hitbox.overlaps(&play_zone) && thing.hitbox.y < self.camera.target.y || thing.should_destroy()
                {
                    remove.push(i);
                }
            }
            for i in remove.iter().rev() {
                self.fling_things.swap_remove(*i);
            }
        }

        if let Some(lightning) = &self.lightning {
            if lightning.should_destroy() {
                self.lightning = None;
            }
        }
    }
}

impl Scene for GameScene {
    fn handle_input(&mut self) {
        update_inputs(&mut self.inputs, &self.bindings);
        if !self.mouse_captured && is_mouse_button_pressed(MouseButton::Left) {
            self.mouse_captured = true;
            show_mouse(false); // does not actually work :(
            set_cursor_grab(true);
        }
        if is_key_down(KeyCode::Escape) {
            self.mouse_captured = false;
            show_mouse(true);
            set_cursor_grab(false);
        }
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

        self.generator.generate(
            &self.camera,
            &self.player,
            &mut self.birds,
            &mut self.lightning,
            elapsed,
        );

        if let Some(ref mut lightning) = self.lightning {
            lightning.update(&self.camera, elapsed);
        }

        self.check_collisions();

        if self.player.position.y + 50.0 >= self.ground_position {
            self.player.land();
        } else {
            self.time += elapsed;
        }

        self.clean_up();
        SceneAction::Continue
    }

    fn render(&self, assets: &mut Assets) {
        draw_background(&self.camera, assets);
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
        self.player.draw(assets);
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

        let fuel_pos = self.camera.screen_to_world(Vec2::new(15.0, 4.0));
        let fuel_texture = match self.player.fuel {
            0 => assets.meter_empty,
            1 => assets.meter_1,
            2 => assets.meter_2,
            _ => assets.meter_full,
        };
        draw_texture(fuel_texture, fuel_pos.x, fuel_pos.y, WHITE);

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
