use specs::{Builder, Component, DispatcherBuilder, ReadStorage, WriteStorage, System, DenseVecStorage, World, WorldExt, RunNow};
use specs::shred::{Dispatcher};

#[derive(Debug)]
pub struct Collision {
    pub circle: bool,
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}

