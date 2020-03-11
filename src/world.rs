use specs::{World, WorldExt}; // Builder, Component, ReadStorage, System, VecStorage, RunNow
use specs::shred::{Dispatcher, DispatcherBuilder};
use rand::prelude::*;

use crate::resources::{add_resources};
use crate::components::{register_components}; // Position, Velocity,
use crate::entities::player::{PlayerEntity};
use crate::entities::ball::{Ball};
use crate::systems::*;

fn init_world(world: &mut World) {
    let mut rng = rand::thread_rng();
    const POSX_RANGE: f32 = 900.0;
    const POSY_RANGE: f32 = 600.0;
    const VELX_RANGE: f32 = 375.0;
    const VELY_RANGE: f32 = 125.0;

    PlayerEntity::build(world, 400.0, 20.0);

    for i in 0..75 {
        let x: f32 = rng.gen::<f32>() * POSX_RANGE;
        let y: f32 = rng.gen::<f32>() * POSY_RANGE;
        let vx: f32 = (rng.gen::<f32>() * VELX_RANGE) - (VELX_RANGE / 2.0);
        let vy: f32 = (rng.gen::<f32>() * VELY_RANGE) - (VELY_RANGE / 2.0);
        // build ball entity and add to world
        if i % 5 > 0 {
            Ball::build(world, x, y, vx, vy);
        }
        else {
            Ball::build_static(world, x, y);
        }
        
    }
}

pub fn create_world<'a>() -> World {
    let mut world = World::new();
    
    add_resources(&mut world);

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
    .with(GravSys, "grav_sys", &["update_pos"])
    //.with(SumSys, "sum_sys", &["update_pos"])
    .build();

    dispatcher
}