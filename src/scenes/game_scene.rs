use enum_map::{enum_map, Enum, EnumMap};
use macroquad::{audio::play_sound_once, prelude::*};

use super::end_scece::EndScene;
use crate::{
    assets::Assets,
    bird::Bird,
    cursor::Cursor,
    fling::FlingThing,
    level_gen::{self, Difficulty, Generator},
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

#[derive(Enum, Debug)]
pub enum Sound {
    Hit,
    Lightning,
    Fling,
    Boost,
    End,
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
    end_timer: f32,
    sounds: EnumMap<Sound, bool>,

    camera: Camera2D,
    inputs: EnumMap<Input, bool>,
    bindings: EnumMap<Input, Vec<KeyCode>>,
}

impl GameScene {
    pub fn new(difficulty: Difficulty) -> Box<Self> {
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
        let ground_position = 2000.0;

        let scene = Self {
            player: Player::new(vec2(screen_width() / 2.0, 10.0)),
            cursor: Cursor::new(),
            generator: Generator::new(difficulty),
            fling_things: level_gen::generate_fling_things(ground_position),
            birds: Vec::new(),
            lightning: None,
            time: 0.0,
            ground_position,
            mouse_captured: true,
            end_timer: 0.0,
            sounds: EnumMap::default(),

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
                self.player.bird_collision(&mut self.sounds);
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
                self.player.lightning_collision(&mut self.sounds);
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
        for (_, play) in self.sounds.iter_mut() {
            *play = false;
        }
        if self.end_timer > 0.0 {
            self.end_timer -= elapsed;
            return if self.end_timer <= 0.0 {
                SceneAction::Replace(EndScene::new(self.time))
            } else {
                SceneAction::Continue
            };
        }
        self.player.update(&self.inputs, &mut self.sounds, elapsed);
        self.cursor
            .update(&self.camera, &mut self.fling_things, &mut self.sounds);
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
            lightning.update(&self.camera, &mut self.sounds, elapsed);
        }

        self.check_collisions();

        if self.player.position.y + 50.0 >= self.ground_position {
            self.player.land();
            self.sounds[Sound::End] = true;
            self.end_timer = 2.0;
        } else {
            self.time += elapsed;
        }

        self.clean_up();
        SceneAction::Continue
    }

    fn render(&self, assets: &mut Assets) {
        draw_texture(assets.background, 0.0, -640.0, WHITE);
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

        let text_pos = self.camera.screen_to_world(Vec2::new(screen_width() - 150.0, 30.0));
        draw_text_ex(
            &format_time(self.time),
            text_pos.x,
            text_pos.y,
            TextParams {
                font: assets.font,
                font_size: 35,
                ..Default::default()
            },
        );

        let fuel_pos = self.camera.screen_to_world(Vec2::new(15.0, 4.0));
        let fuel_texture = match self.player.fuel {
            0 => assets.meter_empty,
            1 => assets.meter_1,
            2 => assets.meter_2,
            _ => assets.meter_full,
        };
        draw_texture(fuel_texture, fuel_pos.x, fuel_pos.y, WHITE);

        self.cursor.draw();

        for (sound, play) in self.sounds.iter() {
            if *play {
                match sound {
                    Sound::Hit => play_sound_once(assets.sfx_hit),
                    Sound::Lightning => play_sound_once(assets.sfx_lightning),
                    Sound::Fling => play_sound_once(assets.sfx_fling),
                    Sound::Boost => play_sound_once(assets.sfx_boost),
                    Sound::End => play_sound_once(assets.sfx_end),
                }
            }
        }
    }
}

pub fn format_time(seconds: f32) -> String {
    let minutes = f32::trunc(seconds / 60.0);
    let seconds = seconds - 60.0 * minutes;

    format!("{:>02}:{:>05.2}", minutes, seconds)
}
