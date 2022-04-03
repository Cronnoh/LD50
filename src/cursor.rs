use macroquad::prelude::*;

use crate::fling::FlingThing;

pub struct Cursor {
    position: Vec2,
    click_position: Option<Vec2>,
    selected_index: Option<usize>,
}

impl Cursor {
    pub fn new() -> Self {
        let mouse_position = mouse_position();
        Self {
            position: Vec2::new(mouse_position.0, mouse_position.1),
            click_position: None,
            selected_index: None,
        }
    }

    pub fn update(&mut self, camera: &Camera2D, fling_things: &mut Vec<FlingThing>) {
        (self.position.x, self.position.y) = mouse_position();
        self.position = camera.screen_to_world(self.position);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.selected_index = fling_things
                .iter()
                .position(|thing| thing.hitbox.contains(self.position));
            self.click_position = Some(self.position);
        } else if !is_mouse_button_down(MouseButton::Left) {
            if let (Some(index), Some(point)) = (self.selected_index, self.click_position) {
                fling_things[index].fling(point - self.position);
            }
            self.click_position = None;
            self.selected_index = None;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 10.0, RED);

        if let Some(click_position) = self.click_position {
            draw_line(
                self.position.x,
                self.position.y,
                click_position.x,
                click_position.y,
                3.0,
                MAROON,
            )
        }
    }

    pub fn has_selected(&self) -> bool {
        self.selected_index.is_some()
    }
}
