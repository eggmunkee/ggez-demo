use ggez::{Context};
use specs::{World, WorldExt}; // Builder, Component, ReadStorage, System, VecStorage, RunNow
use specs::shred::{Dispatcher, DispatcherBuilder};
use rand::prelude::*;

use crate::resources::{add_resources,GameStateResource};
use crate::components::{register_components}; // Position, Velocity,
use crate::entities::player::{PlayerEntityBuilder};
use crate::entities::ball::{BallBuilder};
use crate::systems::*;

// Initialize world entities and state
fn init_world(world: &mut World, ctx: &mut Context) {
    let mut rng = rand::thread_rng();
    const POSX_RANGE: f32 = 900.0;
    const POSY_RANGE: f32 = 600.0;
    const VELX_RANGE: f32 = 95.0;
    const VELY_RANGE: f32 = 75.0;

    PlayerEntityBuilder::build(world, ctx, 400.0, 20.0);

    for i in 0..15 {
        let x: f32 = rng.gen::<f32>() * POSX_RANGE;
        let y: f32 = rng.gen::<f32>() * POSY_RANGE;
        let vx: f32 = (rng.gen::<f32>() * VELX_RANGE) - (VELX_RANGE / 2.0);
        let vy: f32 = (rng.gen::<f32>() * VELY_RANGE) - (VELY_RANGE / 2.0);
        // build ball entity and add to world
        if i % 11 < 5 {
            // if i % 11 == 0 {
            //     BallBuilder::build(world, ctx, x, y, vx, vy);
            // }
            // else {
            BallBuilder::build_collider(world, ctx, x, y, vx, vy, 40.0, 0.002);
            //}
        }
        else {
            BallBuilder::build_static_collider(world, ctx, x-40.0, y, 40.0, 0.001);
            BallBuilder::build_static_collider(world, ctx, x, y, 40.0, 0.001);
            BallBuilder::build_static_collider(world, ctx, x+40.0, y, 40.0, 0.001);
        }
        
    }
}

// Build world by loading resources, components, and calling init_world
pub fn create_world<'a>(ctx: &mut Context, game_state_res: GameStateResource) -> World {
    let mut world = World::new();
    
    world.insert(game_state_res);

    add_resources(&mut world, ctx);

    // Register all components
    register_components(&mut world);

    // Create initial world entities
    init_world(&mut world, ctx);

    world
}

// Create the dispatcher for the world systems
pub fn create_dispatcher<'a>() -> Dispatcher<'a,'a> {
    // build disptacher by including all needed systems
    let dispatcher = DispatcherBuilder::new()
    // apply inputs to entities that are player controlled
    .with(InputSystem::new(), "input_system", &[])
    // apply gravity to entities
    .with(GravSys, "grav_sys", &["input_system"])
    // handle entity interactions
    .with(InterActorSys::new(), "interactor_sys", &["grav_sys"])
    // update entity positions by velocity
    .with(UpdatePos { t_delta: core::time::Duration::new(1,0) }, "update_pos", &["interactor_sys"])
    // reports entity status
    //.with(SumSys, "sum_sys", &["update_pos"])
    .build();

    dispatcher
}