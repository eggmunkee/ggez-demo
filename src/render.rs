
use ggez::{Context, GameResult};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,set_window_title};

use specs::{WorldExt};
use rand::prelude::*;

use crate::components::{Position,DisplayComp};
use crate::game_state::{GameState};

// pub mod circle;
// pub mod square;
pub struct Renderer;

impl Renderer {
    pub fn circs(game_state: &mut GameState, ctx: &mut Context) -> Result<(),()> {
        let mut rng = rand::thread_rng();

        

        match (graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            20.0,
            4.0,
            graphics::WHITE,
        ),
        graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::from([-10.0,-10.0,10.0,10.0]),
            graphics::BLACK,
        )) {
            (Ok(circle),Ok(rect)) => {
                use specs::join::Join;
                
                // Get Position read storage - access to positions of all entities
                let pos = game_state.world.read_storage::<Position>();
                let disp = game_state.world.read_storage::<DisplayComp>();
                // Get entities list
                let ent = game_state.world.entities();
                let mut draw_ok = true;
                // iterator positions and entities together read-only
                for (pos, ent, disp) in (&pos, &ent, &disp).join() {
                    if disp.circle {
                        let mut col_vals: (u8,u8,u8) = rng.gen();
                        //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
                        if let Err(_) = graphics::draw(ctx, &circle, DrawParam::default()
                                    .dest(na::Point2::new(pos.x, pos.y))
                                    .scale(na::Vector2::new(1.0f32,1.0f32))
                                    .color(Color::from_rgba(col_vals.0,col_vals.1,col_vals.2,200)) ) {
                            draw_ok = false;
                        };    
                    }
                    else {
                        let mut col_vals: (u8,u8,u8) = rng.gen();
                        //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
                        if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x, pos.y),
                                Color::from_rgba(255,255,255,255) )) {
                            draw_ok = false;
                        };  
                    }
                }

                match draw_ok {
                    true => Ok(()),
                    _ => Err(())
                }
                
            },
            (_,_) => Err(())
        }
    }

    pub fn draw(game_state: &mut GameState, ctx: &mut Context) -> GameResult {
        
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let draw_ok = if let Ok(_) = Self::circs(game_state, ctx) {
            true
        }
        else {
            false
        };

        if !draw_ok {
            println!("Draw error occurred");
        }

        let fps = ggez::timer::fps(ctx);
        set_window_title(ctx, format!("GGEZ ~~~ DEMO ({:.1} fps)", &fps).as_str());

        graphics::present(ctx)?;

        Ok(())
    }

}