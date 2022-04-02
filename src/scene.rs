use crate::assets::Assets;

pub trait Scene {
    fn handle_input(&mut self);
    fn update(&mut self, elapsed: f32) -> SceneAction;
    fn render(&self, assets: &mut Assets);
}

pub enum SceneAction {
    Continue,
    _Push(Box<dyn Scene>),
    _Replace(Box<dyn Scene>),
    Pop,
}

pub struct SceneManager {
    stack: Vec<Box<dyn Scene>>,
}

impl SceneManager {
    pub fn new(start_scene: Box<dyn Scene>) -> Self {
        SceneManager {
            stack: vec![start_scene],
        }
    }

    pub fn update(&mut self, assets: &mut Assets, elapsed: f32) {
        let current_scene = match self.stack.last_mut() {
            Some(scene) => scene,
            None => return,
        };

        current_scene.handle_input();
        let action = current_scene.update(elapsed);
        current_scene.render(assets);

        match action {
            SceneAction::Continue => {}
            SceneAction::_Push(x) => self.stack.push(x),
            SceneAction::_Replace(x) => {
                drop(self.stack.pop());
                self.stack.push(x);
            }
            SceneAction::Pop => drop(self.stack.pop()),
        }
    }
}
