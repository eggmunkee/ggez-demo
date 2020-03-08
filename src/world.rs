use specs::{Builder, Component, ReadStorage, System, VecStorage,World, WorldExt, RunNow};
use specs::shred::{Dispatcher, DispatcherBuilder};
use rand::prelude::*;

use crate::components::{Position, Velocity, register_components};
use crate::systems::*;

fn init_world(world: &mut World) {
    let mut rng = rand::thread_rng();
    const posx_range: f32 = 900.0;
    const posy_range: f32 = 600.0;
    const velx_range: f32 = 150.0;
    const vely_range: f32 = 80.0;

    for i in 0..150 {
        let x: f32 = rng.gen::<f32>() * posx_range;
        let y: f32 = rng.gen::<f32>() * posy_range;
        let vx: f32 = (rng.gen::<f32>() * velx_range) - (velx_range / 2.0);
        let vy: f32 = (rng.gen::<f32>() * vely_range) - (vely_range / 2.0);
        // create an object in world
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: vx, y: vy })
        .build();
    }
}

pub fn create_world<'a>() -> World {
    let mut world = World::new();
    
    // Register all components
    register_components(&mut world);

    // Create initial world entities
    init_world(&mut world);

    world
}

pub fn create_dispatcher<'a>() -> Dispatcher<'a,'a> {
    // build disptacher by including all needed systems
    let dispatcher = DispatcherBuilder::new()
    .with(InputSystem::new(), "input_system", &[])
    .with(UpdatePos, "update_pos", &["input_system"])
    //.with(SumSys, "sum_sys", &["update_pos"])
    .build();

    dispatcher
}