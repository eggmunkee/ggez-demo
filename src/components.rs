use specs::{Builder, Component, DispatcherBuilder, ReadStorage, WriteStorage, System, VecStorage, World, WorldExt, RunNow};
use specs::shred::{Dispatcher};

pub mod player;
pub mod collision;
// DEFINE COMMON COMPONENTS

#[derive(Debug)]
pub struct GridLoc {
    pub row: i32,
    pub col: i32,
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub gravity: bool,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct DisplayComp {
    pub circle: bool,
}

impl Component for DisplayComp {
    type Storage = VecStorage<Self>;
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<DisplayComp>();
    
    // sub-module components
    self::player::register_components(world);
}
