
use specs::prelude::*;

use crate::resources::{InputResource,WorldAction};
use crate::components::*;
use crate::components::player::*;

// handle input state to control Players
// every frame, operate on velocity of player components
//  based on InputResource
pub struct InputSystem;
impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem
    }

    fn test(mut v: Vec<(&mut Velocity, &PlayerComponent, Entity)>, input: &InputResource) {

        // handle each input applicable entity
        for inn_v in v.iter_mut() { 
            let (vel, _player, _e) = inn_v;      
            Self::test_inner(inn_v, input);
        }
    }

    // handle input updates from an entity
    fn test_inner(v: &mut (&mut Velocity, &PlayerComponent, Entity), input: &InputResource) {
        let (vel, _player, _e) = v;

        // IS the Input direction Multi-axis, i.e. UP + RIGHT is multi-axis
        let multi_axis = (input.dirs_pressed[0] && (input.dirs_pressed[2] || input.dirs_pressed[3]))
        || (input.dirs_pressed[1] && (input.dirs_pressed[2] || input.dirs_pressed[3]));
        // Single axis vector length
        let mut vec_amt = 35.0;
        // reduce vector length with two axis
        if multi_axis {
            vec_amt = 0.71 * vec_amt;
        }
        vec_amt = vec_amt * 0.05;
        // Apply vector length to velocity X
        if input.dirs_pressed[0] {
            vel.x -= vec_amt;
        }
        else if input.dirs_pressed[1] {
            vel.x += vec_amt;
        }
        else {
            //vel.x = vel.x * 0.98;
        }
        // Apply vector length to velocity Y
        if input.dirs_pressed[2] || input.jump_pressed {
            vel.y -= vec_amt;
        }
        else if input.dirs_pressed[3] {
            vel.y += vec_amt;
        }
        else {
            // if vel.y < 0.0 {
            //     vel.y = vel.y * 0.98;
            // }
        }

        vel.x = vel.x.max(-40.0).min(40.0);
        vel.y = vel.y.max(-80.0);
    }
}
impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteStorage<'a, Velocity>,
                        ReadStorage<'a, PlayerComponent>,
                       Read<'a, InputResource>,
                        Entities<'a>);

    fn run(&mut self, (mut vel, player, mut input, ent): Self::SystemData) {

        // tests collecting storage into vector
        let mut list = (&mut vel, &player, &ent).join().collect::<Vec<_>>();

        if list.len() > 1 {
            println!("More than one player!");
        }
        else if list.len() == 0 {
            println!("No players found!");
        }

        Self::test(list, &*input);

        // iterator over velocities with player components and input
        //for (vel, _player, _e) in list.iter_mut() {        
            //println!("Input proc for player {}", &player.player_name);    

            
        //}
    }
}

// handle ai sim input state to control Npcs
pub struct NpcInputSystem;