
use ggez;
use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::{NumSamples,WindowSetup};
use ggez::graphics::{Color,set_window_title};

use specs::{Builder, Component, DispatcherBuilder, Dispatcher,// ReadStorage, WriteStorage, 
    System, VecStorage, World, WorldExt, RunNow};
use rand::prelude::*;
// =====================================

//use crate::resources::{InputResource};
//use crate::components::{Position,Velocity,DisplayComp};
//use systems::{};
use crate::world::{create_world,create_dispatcher};

use crate::render;
use crate::input::{InputMap};

#[derive(Copy,Clone,Debug)]
pub enum State {
    Running,
    Paused,    
}

//impl Copy for State

// Main game state struct
pub struct GameState<'a> {
    pub current_state: State,
    pub dispatcher: Dispatcher<'a,'a>,
    pub world: World,
}

impl<'a> GameState<'a> {
    pub fn new() -> GameResult<GameState<'static>> {
        
        // Create main state instance with dispatcher and world
        let mut s = GameState { 
            current_state: State::Running,
            dispatcher: create_dispatcher(), 
            world: create_world() 
        };

        // Perform initial dispatch and update world
        println!("Dispatcher & World init");
        s.dispatcher.dispatch(&s.world);
        s.world.maintain();

        Ok(s)
    }
}

impl<'a> GameState<'a> {
    pub fn pause(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Running => { self.current_state = State::Paused; }
            _ => {}
        }        
    }

    pub fn play(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Paused => { self.current_state = State::Running; }
            _ => {}
        }        
    }
}

impl event::EventHandler for GameState<'static> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        // Handle any GameState

        // Get world and dispatcher to increment the entity system
        let world = &mut self.world;
        let dispatcher = &mut self.dispatcher;
        // Call update on the world event dispatcher
        dispatcher.dispatch(&world);
        // Update the world state after dispatch changes
        world.maintain();

        // always ok result
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Call rendering module        
        render::Renderer::draw(self, ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        _repeat: bool,
    ) {
        InputMap::key_down(&mut self.world, ctx, keycode, keymod);
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
    ) {
        InputMap::key_up(&mut self.world, ctx, keycode, keymod);
    }

}