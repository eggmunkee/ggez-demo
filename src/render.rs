
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,set_window_title};

use specs::{WorldExt};
use rand::prelude::*;

//use crate::resources::{ImageResources};
use crate::components::{Position,DisplayComp,DisplayCompType};
use crate::components::player::{CharacterDisplayComponent};
use crate::game_state::{GameState,State};

// pub mod circle;
// pub mod square;
pub struct Renderer;

impl Renderer {
    pub fn circs(game_state: &mut GameState, ctx: &mut Context) -> GameResult<()> {
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
            graphics::WHITE,
        )) {
            (Ok(circle),Ok(rect)) => {
                use specs::join::Join;
                
                // Get Position read storage - access to positions of all entities
                //let mut image_res = game_state.world.fetch_mut::<ImageResources>();
                let pos = game_state.world.read_storage::<Position>();
                let disp = game_state.world.read_storage::<DisplayComp>();
                // Get entities list
                let ent = game_state.world.entities();
                let mut draw_ok = true;
                // iterator positions and entities together read-only
                for (pos, ent, disp) in (&pos, &ent, &disp).join() {
                    println!("Display type: {:?}", disp);
                    if let DisplayCompType::DrawCircle = disp.display_type {
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
                            let mut col_vals: (u8,) = rng.gen();
                            //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
                            if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x, pos.y),
                                    Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255) )) {
                                draw_ok = false;
                            };  
                            // if let Ok(image_ref) = image_res.image_ref(String::from("/icon.png")) {
                            //     if let Err(_) = graphics::draw(ctx, image_ref, (na::Point2::new(pos.x+15.0, pos.y+15.0),)) {
                            //         draw_ok = false;
                            //     }
                            // }
                            // else {
                            //     draw_ok = false;
                            // }

                        }
                    }
                    else if let DisplayCompType::DrawSelf(draw_func) = disp.display_type {
                        //draw_func(game_state, &ent, ctx);
                        //rend_list.push(disp);
                    }
                }

                let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
                for (char_disp,pos) in (&char_disp,&pos).join() {
                    char_disp.draw(ctx, na::Point2::new(pos.x, pos.y));
                }

                match draw_ok {
                    true => Ok(()),
                    _ => Err(GameError::RenderError(String::from("circs draw error")))
                }
                
            },
            (_,_) => Err(GameError::RenderError(String::from("circs build error")))
        }
    }

    pub fn draw(game_state: &mut GameState, ctx: &mut Context) -> GameResult {
        
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        match &game_state.current_state {
            State::Running => {

                let draw_ok = if let Ok(_) = Self::circs(game_state, ctx) {
                    true
                }
                else {
                    false
                };

                if !draw_ok {
                    println!("Draw error occurred");
                }

            },
            _ => {
                
            }
        }

        let fps = ggez::timer::fps(ctx);
        set_window_title(ctx, format!("GGEZ ~~~ DEMO ({:.1} fps)", &fps).as_str());

        graphics::present(ctx)?;

        Ok(())
    }

}