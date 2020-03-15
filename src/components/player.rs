use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Image,Color};
use ggez::nalgebra as na;
use specs::{Builder, Component, Entity, DenseVecStorage, World, WorldExt, RunNow};
use specs::shred::{Dispatcher};
use rand::prelude::*;

use crate::game_state::{GameState};

#[derive(Debug)]
pub struct PlayerComponent {
    pub player_name: String,
    pub life: i32,
    pub move_ramp_up: f32,
    //pub image: Image, // component owns image
    //pub path: String,
}
impl Component for PlayerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl PlayerComponent {
    pub fn new(ctx: &mut Context, char_img: &String) -> PlayerComponent {

        //let image = Image::new(ctx, char_img.clone()).unwrap();

        PlayerComponent {
            player_name: String::from("playername1"),
            life: 100,
            move_ramp_up: 0.0f32,
            // image: image,
            // path: char_img.clone()
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.player_name = name;
    }
}


#[derive(Debug)]
pub struct CharacterDisplayComponent {
    pub image: Image, // component owns image
    pub path: String,
}
impl Component for CharacterDisplayComponent {
    type Storage = DenseVecStorage<Self>;
}

impl CharacterDisplayComponent {
    pub fn new(ctx: &mut Context, char_img: &String) -> CharacterDisplayComponent {
        let image = Image::new(ctx, char_img.clone()).unwrap();

        CharacterDisplayComponent {
            image: image,
            path: char_img.clone(),
        }
    }
    // pub fn draw(&self, ctx: &mut Context, pos: na::Point2::<f32>) {
    //     let mut rng = rand::thread_rng();
    //     let mut draw_ok = true;
    //     let w = self.image.width();
    //     let h = self.image.height();
    //     let draw_pos = na::Point2::<f32>::new(pos.x - (w as f32 / 2.0), pos.y - (h as f32 / 2.0));
    //     // color part:  ,Color::new(1.0,0.7,0.7,1.0)
    //     if let Err(_) = ggez::graphics::draw(ctx, &self.image, (draw_pos.clone(),)) { // add back x/y pos  //
    //         draw_ok = false;
    //     }

    //     if let Ok(rect) = graphics::Mesh::new_rectangle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         graphics::Rect::from([0.0,0.0,4.0,4.0]),
    //         graphics::WHITE,
    //     ) {
    //         let mut col_vals: (u8,) = rng.gen();
    //         //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
    //         if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x-2.0, pos.y-2.0),
    //                 Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255) )) {
    //             draw_ok = false;
    //         };  
    //     }
    // }
}

// pub trait PlayerRenderTrait {
//     fn draw(&self, ctx: &mut Context, pos: na::Point2::<f32>);
// }

impl super::RenderTrait for CharacterDisplayComponent {
    fn draw(&self, ctx: &mut Context, ent: Option<u32>, pos: na::Point2::<f32>) {
        //println!("PlayerRenderTrait drawing...");
        let mut rng = rand::thread_rng();
        let mut draw_ok = true;
        let w = self.image.width();
        let h = self.image.height();
        let draw_pos = na::Point2::<f32>::new(pos.x - (w as f32 / 2.0), pos.y - (h as f32 / 2.0));
        // color part:  ,Color::new(1.0,0.7,0.7,1.0)
        if let Err(_) = ggez::graphics::draw(ctx, &self.image, (draw_pos.clone(),)) { // add back x/y pos  //
            draw_ok = false;
        }

        if let Ok(rect) = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::from([0.0,0.0,4.0,4.0]),
            graphics::WHITE,
        ) {
            let mut col_vals: (u8,) = rng.gen();
            //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
            if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x-2.0, pos.y-2.0),
                    Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255) )) {
                draw_ok = false;
            };  
        }
    }
}




// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<PlayerComponent>();
    world.register::<CharacterDisplayComponent>();
}