
use specs::{Builder, Component, ReadStorage, System, VecStorage,World, WorldExt, RunNow};


#[derive(Default, Debug)]
pub struct InputResource {
    pub dirs_pressed: [bool;4],
    pub jump_pressed: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_down: [bool;3],
}

impl InputResource {
    pub fn set_left(&mut self, press: bool) {
        self.dirs_pressed[0] = press;
    }
    pub fn set_right(&mut self, press: bool) {
        self.dirs_pressed[1] = press;
    }
    pub fn set_up(&mut self, press: bool) {
        self.dirs_pressed[2] = press;
    }
    pub fn set_down(&mut self, press: bool) {
        self.dirs_pressed[3] = press;
    }
    pub fn set_jump(&mut self, press: bool) {
        self.jump_pressed = press;
    }
    pub fn set_mouse_pos(&mut self, mouse_x: f32, mouse_y: f32) {
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;
    }
    pub fn set_mouse_x(&mut self, mouse_x: f32) {
        self.mouse_x = mouse_x;
    }
    pub fn set_mouse_y(&mut self, mouse_y: f32) {
        self.mouse_y = mouse_y;
    }
    pub fn set_mouse_down(&mut self, mouse_down: bool, button_index: usize) {
        if button_index < 3 {
            self.mouse_down[button_index] = mouse_down;
        }
    }
}

pub fn add_resources(world: &mut World) {
    world.insert(InputResource { 
        dirs_pressed: [false,false,false,false],
        jump_pressed: false,
        mouse_x: 0.0,
        mouse_y: 0.0,
        mouse_down: [false,false,false],
    });
}