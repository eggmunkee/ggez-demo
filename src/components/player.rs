
use specs::{Builder, Component, DispatcherBuilder, ReadStorage, WriteStorage, System, VecStorage, DenseVecStorage, World, WorldExt, RunNow};
use specs::shred::{Dispatcher};

#[derive(Debug)]
pub struct PlayerComponent {
    pub player_name: String,
    pub life: i32,
    pub move_ramp_up: f32,
}
impl Component for PlayerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl PlayerComponent {
    pub fn new() -> PlayerComponent {
        PlayerComponent {
            player_name: String::from("playername1"),
            life: 100,
            move_ramp_up: 0.0f32
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.player_name = name;
    }
}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<PlayerComponent>();
}