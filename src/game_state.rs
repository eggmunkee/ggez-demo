
use ggez;
use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::{NumSamples,WindowSetup};
use ggez::graphics::{Color};

use specs::{Builder, Component, DispatcherBuilder, Dispatcher,// ReadStorage, WriteStorage, 
    System, VecStorage, World, WorldExt, RunNow};
use rand::prelude::*;
// =====================================

use crate::components::{Position,Velocity};
//use systems::{};
use crate::world::{create_world,create_dispatcher};


// Main game state struct

pub struct GameState<'a> {
    pub dispatcher: Dispatcher<'a,'a>,
    pub world: World,
}

impl<'a> GameState<'a> {
    pub fn new() -> GameResult<GameState<'static>> {
        
        // Create main state instance with dispatcher and world
        let mut s = GameState { 
            dispatcher: create_dispatcher(), 
            world: create_world() 
        };

        // Perform initial dispatch and update world
        println!("Dispatcher init");
        s.dispatcher.dispatch(&s.world);
        s.world.maintain();
        //println!("After initial dispatch & maintain...");

        Ok(s)
    }
}

impl<'a> GameState<'a> {
    fn circs(&self, ctx: &mut Context) -> Result<(),()> {
        let mut rng = rand::thread_rng();

        match graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            20.0,
            4.0,
            graphics::WHITE,
        ) {
            Ok(circle) => {
                use specs::join::Join;
                
                // Get Position read storage - access to positions of all entities
                let pos = self.world.read_storage::<Position>();
                // Get entities list
                let ent = self.world.entities();
                let mut draw_ok = true;
                // iterator positions and entities together read-only
                for (pos, ent) in (&pos, &ent).join() {
                    let mut col_vals: (u8,u8,u8) = rng.gen();
                    //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
                    if let Err(_) = graphics::draw(ctx, &circle, (na::Point2::new(pos.x, pos.y),
                            Color::from_rgba(col_vals.0,col_vals.1,col_vals.2,200) )) {
                        draw_ok = false;
                    };
                }

                match draw_ok {
                    true => Ok(()),
                    _ => Err(())
                }
                
            },
            _ => Err(())
        }
    }

}

impl event::EventHandler for GameState<'static> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        // Get world and dispatcher to increment the entity system
        let world = &mut self.world;
        let dispatcher = &mut self.dispatcher;
        dispatcher.dispatch(&world);
        world.maintain();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let draw_ok = if let Ok(_) = self.circs(ctx) {
            true
        }
        else {
            false
        };

        if !draw_ok {
            println!("Draw error occurred");
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );

        if keycode == ggez::event::KeyCode::Escape {
            ggez::event::quit(ctx);
        }
    }

}