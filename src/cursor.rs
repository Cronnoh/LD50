use macroquad::prelude::*;

pub struct Cursor {
    position: Vec2,
    click_position: Option<Vec2>,
}

impl Cursor {
    pub fn new() -> Self {
        let mouse_position = mouse_position();
        Self {
            position: Vec2::new(mouse_position.0, mouse_position.1),
            click_position: None,
        }
    }

    pub fn update(&mut self, camera: &Camera2D) {
        (self.position.x, self.position.y) = mouse_position();
        self.position = camera.screen_to_world(self.position);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.click_position = Some(self.position);
        } else if !is_mouse_button_down(MouseButton::Left) {
            self.click_position = None;
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
}
