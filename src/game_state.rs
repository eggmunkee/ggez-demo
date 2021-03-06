
use std::collections::hash_map::*;
use ggez;
use ggez::graphics;
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
//use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult, GameError};
use ggez::conf::{NumSamples,WindowSetup,WindowMode};
use ggez::graphics::{Rect,Color,Image,set_window_title};

use specs::{Builder, Component, DispatcherBuilder, Dispatcher,// ReadStorage, WriteStorage, 
    System, VecStorage, World, WorldExt, RunNow};
use rand::prelude::*;
// =====================================

use crate::resources::{InputResource,WorldAction,GameStateResource};
//use crate::components::{Position,Velocity,DisplayComp};
//use systems::{};
use crate::world::{create_world,create_dispatcher};
//use crate::resources::{ImageResources};

use crate::render;
use crate::input::{InputMap,MouseInput};

#[derive(Copy,Clone,Debug)]
pub enum State {
    Running,
    Paused,    
}

//impl Copy for State

// Main game state struct
pub struct GameState<'a> {
    pub current_state: State,
    pub window_w: f32,
    pub window_h: f32,
    pub dispatcher: Dispatcher<'a,'a>,
    pub world: World,
    pub font: graphics::Font,
    //pub image_lookup: HashMap<String,usize>,
    //pub images: Vec<Image>
    pub paused_text: graphics::Text,
}

impl<'a> GameState<'a> {
    pub fn new(ctx: &mut Context, window_mode: WindowMode) -> GameResult<GameState<'static>> {

        // Create game state related to window size/mode
        let (win_w, win_h) = ggez::graphics::drawable_size(ctx);
        let game_state_resource = GameStateResource {
            window_w: win_w, window_h: win_h, window_mode: window_mode,
            stop_double: false,
        };

        let font = graphics::Font::new(ctx, "/FreeMonoBold.ttf")?;
        let text = graphics::Text::new(("PAUSED", font, 52.0));

        // Create main state instance with dispatcher and world
        let mut s = GameState { 
            current_state: State::Running,
            window_w: win_w,
            window_h: win_h,
            dispatcher: create_dispatcher(), 
            world: create_world(ctx, game_state_resource),
            font: font,
            // image_lookup: HashMap::<String,usize>::new(),
            // images: Vec::<Image>::new(),
            paused_text: text,
        };
        //s.pause();

        // Perform initial dispatch and update world
        println!("Dispatcher & World init");
        s.dispatcher.dispatch(&s.world);
        s.world.maintain();

        // Tests adding images to the image resources Resource
        // if let Some(img_res) = s.world.get_mut::<ImageResources>() {
        //     let img_path = String::from("/icon.png");
        //     // check for image existence
        //     //println!("ImageResources has {}? {}", &img_path, &(img_res.has_image(img_path.clone())) );

        //     // load image once (if not set)
        //     img_res.load_image(img_path.clone(), ctx);
        //     //println!("ImageResources has {}? {}", &img_path, &(img_res.has_image(img_path.clone())) );

        //     // get image reference from path
        //     let img : &Image = img_res.image_ref(img_path, ctx).unwrap();
        //     println!("Image: {:?}", img);
        //     drop(img);

        // }

        Ok(s)
    }

    // pub fn has_image(&mut self, path:String) -> bool {
    //     return self.image_lookup.contains_key(&path);
    // }

    // pub fn load_image(&mut self, path:String, ctx: &mut Context) -> GameResult<()> {
    //     let entry = self.image_lookup.entry(path.clone());
    //     if let Entry::Vacant(_) = entry {
    //         let image = Image::new(ctx, path.clone())?;
    //         let new_idx = self.images.len();
    //         self.images.push(image);
    //         self.image_lookup.insert(path.clone(), new_idx);
    //         //()
    //     }
    //     Ok(()) // ok if already loaded
    // }

    // pub fn image_ref(&mut self, path:String) -> GameResult<&mut Image> {
        
    //     //self.load_image(path.clone(), ctx)?;

    //     match self.image_lookup.entry(path.clone()) {
    //         Entry::Occupied(o) => {
    //             //let o = o;
    //             let index = o.get().clone();
    //             let image = &mut self.images[index];
    //             Ok(image)
    //         },
    //         _ => Err(GameError::ResourceLoadError("Got image_ref for missing image".to_string()))
    //     }
    // }
}

impl<'a> GameState<'a> {
    #[allow(dead_code)]
    pub fn pause(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Running => { self.current_state = State::Paused; }
            _ => {}
        }        
    }
    #[allow(dead_code)]
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

        // Yield process to os
        ggez::timer::yield_now();
   
        // Only update world state when game is running (not paused)
        match &self.current_state {
            State::Running => {

                // Get world and dispatcher to increment the entity system
                let world = &mut self.world;
                let dispatcher = &mut self.dispatcher;
                // Call update on the world event dispatcher
                dispatcher.dispatch(&world);
                // Update the world state after dispatch changes
                world.maintain();

            },
            _ => {
                
            }
        }

        // always ok result
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Call rendering module        

        render::Renderer::render_frame(self, &self.world, ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let button_index = match button {
            MouseButton::Left => {
                Some(0usize)
            },
            MouseButton::Middle => {
                Some(1)
            },
            MouseButton::Right => {
                Some(2)
            }
            _ => None
        };
        if let Some(index) = button_index {
            InputMap::mouse_button_down(&mut self.world, ctx, index.clone());
        }
        //self.mouse_down = true;
        //println!("Mouse button pressed: {:?}, x: {}, y: {}, button index: {:?}", button, x, y, button_index);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let button_index = match button {
            MouseButton::Left => {
                Some(0usize)
            },
            MouseButton::Middle => {
                Some(1)
            },
            MouseButton::Right => {
                Some(2)
            }
            _ => None
        };
        if let Some(index) = button_index {
            InputMap::mouse_button_up(&mut self.world, ctx, index.clone());
        }
        
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
        // if self.mouse_down {
        //     self.pos_x = x;
        //     self.pos_y = y;
        // }
        InputMap::mouse_set_pos(&mut self.world, _ctx, x, y);
        // println!(
        //     "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
        //     x, y, xrel, yrel
        // );
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        println!("Mousewheel event, x: {}, y: {}", x, y);
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

        if keycode == KeyCode::P {
            match self.current_state {
                State::Paused => {
                    self.play();
                },
                State::Running => {
                    self.pause();
                }
            }
        }
        else if keycode == KeyCode::J {
            // Get world action if any
            //println!("Processing AddCircle action");
            crate::entities::ball::BallBuilder::build_collider(&mut self.world, ctx, 100.0, 400.0, -2.0, 0.0, 20.0, 0.05);

        }

        InputMap::key_up(&mut self.world, ctx, keycode, keymod);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
        println!("Text input: {}", ch);
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        println!("Resized: {}, {}", &width, &height);


        // set resource w/h
        let mut game_state_writer = self.world.fetch_mut::<GameStateResource>();

        if game_state_writer.stop_double {
            game_state_writer.stop_double = false;
        }
        else {
            self.window_w = width;
            self.window_h = height;
    
            game_state_writer.window_w = width;
            game_state_writer.window_h = height;
            game_state_writer.stop_double = true;
    
            let mut mode = game_state_writer.window_mode;
    
            mode.width = width;
            mode.height = height;
            println!("New window mode {:?}", &mode);
            //ggez::graphics::set_mode(ctx, mode);
    
            ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height));
    
    
    
        }

        drop(game_state_writer);
        
    }
}