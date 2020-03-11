// external crates
use ggez;
use ggez::event;
use ggez::{GameResult};

// ================== ROOT MODULES ========================

mod conf;
// Builders for entity types
mod entities;
// Components available to entities
mod components;
// Shared world data
mod resources;
// Systems which process world state updates
mod systems;
// Sets up the world
mod world;
// Render methods for app
mod render;
// Input key mapping from codes to actions, handle actions
mod input;
// creates game state with world and dispatcher, handles event loop
//   Update, Draw, KeyDown KeyUp, etc.
//   Events are forwarded to specs dispatcher and render/input modules
mod game_state;

use crate::conf::*;

// ======================== MAIN INIT APP ============================

// Do setup and start main event loop
pub fn main() -> GameResult {
    // get ggez context build - builds window app
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(get_window_setup())
        .window_mode(get_window_mode());
    // build
    let (ctx, event_loop) = &mut cb.build()?;
    // create app's state
    let state = &mut crate::game_state::GameState::new()?;
    // run event loop
    event::run(ctx, event_loop, state)
}
