//! The simplest possible example that does something.

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::{NumSamples,WindowSetup};

use specs::{Builder, Component, DispatcherBuilder, ReadStorage, System, VecStorage,World, WorldExt, RunNow};
use specs::shred::{Dispatcher};

use rand;

mod entities;
mod components;
mod systems;
mod world;
mod game_state;

// use components::{Position,Velocity};
// //use systems::{};
// use world::{create_world,create_dispatcher};

// struct GameState<'a> {
//     dispatcher: Dispatcher<'a,'a>,
//     world: World,
// }

// impl<'a> GameState<'a> {
//     fn new() -> GameResult<GameState<'static>> {
        
//         // Create main state instance with dispatcher and world
//         let mut s = GameState { 
//             dispatcher: create_dispatcher(), 
//             world: create_world() 
//         };

//         // Perform initial dispatch and update world
//         println!("Before initial dispatch & maintain...");
//         s.dispatcher.dispatch(&s.world);
//         s.world.maintain();
//         println!("After initial dispatch & maintain...");

//         Ok(s)
//     }
// }

// impl<'a> GameState<'a> {
//     fn circs(&self, ctx: &mut Context) -> Result<(),()> {
//         match graphics::Mesh::new_circle(
//             ctx,
//             graphics::DrawMode::fill(),
//             na::Point2::new(0.0, 0.0),
//             40.0,
//             4.0,
//             graphics::WHITE,
//         ) {
//             Ok(circle) => {
//                 use specs::join::Join;
                
//                 // Get Position read storage - access to positions of all entities
//                 let pos = self.world.read_storage::<Position>();
//                 // Get entities list
//                 let ent = self.world.entities();
//                 // iterator positions and entities together read-only
//                 for (pos, ent) in (&pos, &ent).join() {
//                     println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
//                     graphics::draw(ctx, &circle, (na::Point2::new(pos.x, pos.y),));
//                 }

//                 Ok(())
//             },
//             _ => Err(())
//         }
//     }

// }

// impl event::EventHandler for GameState<'static> {
//     fn update(&mut self, _ctx: &mut Context) -> GameResult {

//         // Get world and dispatcher to increment the entity system
//         let world = &mut self.world;
//         let dispatcher = &mut self.dispatcher;
//         dispatcher.dispatch(&world);
//         world.maintain();

//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult {
//         graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

//         self.circs(ctx);

//         graphics::present(ctx)?;
//         Ok(())
//     }
// }

pub fn main() -> GameResult {
    // get ggez context build - builds window app
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(WindowSetup {
            title: "GGEZ ~~~ DEMO".to_owned(),
            samples: NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        });
    // build
    let (ctx, event_loop) = &mut cb.build()?;
    // create app's state
    let state = &mut crate::game_state::GameState::new()?;
    // run event loop
    event::run(ctx, event_loop, state)
}
