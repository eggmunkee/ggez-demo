use std::fmt;
use ggez::{Context,GameResult};
use specs::{Builder, Component,Entity, DispatcherBuilder, ReadStorage, WriteStorage, System, VecStorage, World, WorldExt, RunNow};
use specs::shred::{Dispatcher};

use crate::game_state::{GameState};

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

//pub type draw_fn = fn(game_state: &mut GameState, entity: &Entity, ctx: &mut Context) -> GameResult<()>;

pub enum DisplayCompType {
    DrawCircle,
    DrawSelf
}
impl fmt::Debug for DisplayCompType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //let ds = f.debug_struct("DisplayCompType");
        match self {
            DisplayCompType::DrawCircle => {
                f.write_str("DrawCircle")?;
            },
            DisplayCompType::DrawSelf => {
                f.write_str("DrawSelf")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DisplayComp {
    pub circle: bool,
    pub display_type: DisplayCompType,
}

impl DisplayComp {
    fn draw_self(game_state: &mut GameState, entity: &Entity, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

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
    self::collision::register_components(world);
    self::player::register_components(world);
}
