
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
}
impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteStorage<'a, Velocity>,
                        ReadStorage<'a, PlayerComponent>,
                       Write<'a, InputResource>,
                        Entities<'a>);

    fn run(&mut self, (mut vel, player, mut input, ent): Self::SystemData) {

        // iterator over velocities with player components and input
        for (vel, _player, _e) in (&mut vel, &player, &ent).join() {        
            //println!("Input proc for player {}", &player.player_name);    

            // IS the Input direction Multi-axis, i.e. UP + RIGHT is multi-axis
            let multi_axis = (input.dirs_pressed[0] && (input.dirs_pressed[2] || input.dirs_pressed[3]))
                || (input.dirs_pressed[1] && (input.dirs_pressed[2] || input.dirs_pressed[3]));
            // Single axis vector length
            let mut vec_amt = 120.0;
            // reduce vector length with two axis
            if multi_axis {
                vec_amt = 0.77 * vec_amt;
            }
            // Apply vector length to velocity X
            if input.dirs_pressed[0] {
                vel.x = -vec_amt;
            }
            else if input.dirs_pressed[1] {
                vel.x = vec_amt;
            }
            else {
                vel.x = vel.x * 0.90;
            }
            // Apply vector length to velocity Y
            if input.dirs_pressed[2] || input.jump_pressed {
                vel.y = -vec_amt;
            }
            else if input.dirs_pressed[3] {
                vel.y = vec_amt;
            }
            else {
                vel.y = vel.y * 0.90;
            }
        }
    }
}

// handle ai sim input state to control Npcs
pub struct NpcInputSystem;