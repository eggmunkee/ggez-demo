
use specs::{Builder, Component, ReadStorage, System, VecStorage,World, WorldExt, RunNow};


#[derive(Default, Debug)]
pub struct InputResource {
    pub dirs_pressed: [bool;4],
    pub jump_pressed: bool,
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
}

pub fn add_resources(world: &mut World) {
    world.insert(InputResource { 
        dirs_pressed: [false,false,false,false],
        jump_pressed: false
    });
}