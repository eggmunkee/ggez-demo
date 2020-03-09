
use specs::{
    //Builder, DispatcherBuilder,
    Entities,
    ReadStorage, WriteStorage, System, //VecStorage, 
    Read,
};
//use specs::shred::{Dispatcher};

use crate::resources::{InputResource};
use crate::components::{Position,Velocity,GridLoc};
use crate::components::player::{PlayerComponent};

/**** SYSTEMS *********************************/

// UpdatePos system - can update the position of each entity by the velocity amount
pub struct UpdatePos;

impl UpdatePos {
    fn box_pos(pos: &mut Position, vel: &mut Velocity) {
        let (min_x, max_x, min_y, max_y) = (10.0, 790.0, 10.0, 590.0);
        if pos.x < min_x {
            pos.x = min_x;
            if vel.x < 0.0 { vel.x = -vel.x; }
        }
        else if pos.x > max_x {
            pos.x = max_x;
            if vel.x > 0.0 { vel.x = -vel.x; }
        }
        if pos.y < min_y {
            pos.y = min_y;
            if vel.y < 0.0 { vel.y = -vel.y; }
        }
        else if pos.y > max_y {
            pos.y = max_y;
            if vel.y > 0.0 { vel.y = -vel.y; }
        }
    }

    #[allow(dead_code)]
    pub fn get_grid_locs(pos: &mut Position) -> Vec<GridLoc> {
        let gl_vec = Vec::<GridLoc>::new();



        gl_vec
    }
}

impl<'a> System<'a> for UpdatePos {
    type SystemData = (WriteStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (mut vel, mut pos): Self::SystemData) {
        use specs::Join;
        //let mut i = 0;
        for (vel, pos) in (&mut vel, &mut pos).join() {
            // update positions by velocity vectors
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
            // Constrain position to a box (hardcoded atm)
            //  and reverse velocity when going out of bounds
            self::UpdatePos::box_pos(pos, vel);
        }
    }
}


// UpdatePos system - can update the position of each entity by the velocity amount
pub struct GravSys;

impl GravSys {
   
}

impl<'a> System<'a> for GravSys {
    type SystemData = WriteStorage<'a, Velocity>;

    fn run(&mut self, mut vel: Self::SystemData) {
        use specs::Join;
        for vel in (&mut vel).join() {
            // update velocity if gravity is applied
            if vel.gravity {
                vel.y += 9.8 * 0.05;
            }
        }
    }
}


// SumSym - summarizes the pos & vel of all entities, which have both components
pub struct SumSys;
impl<'a> System<'a> for SumSys {
    type SystemData = (ReadStorage<'a, Velocity>,
                       ReadStorage<'a, Position>);

    fn run(&mut self, (vel, pos): Self::SystemData) {
        use specs::Join;
        let mut i = 0;
        for (vel, pos) in (&vel, &pos).join() {            
            println!("Ent {}, Pos: {:?} Vel: {:?}", &i, &pos, &vel);
            i += 1;
        }
    }
}

// handle game level commands
#[allow(dead_code)]
pub struct GameCommandSystem;
// handle changes to the game state
#[allow(dead_code)]
pub struct GameStateSys;

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
                       Read<'a, InputResource>,
                        Entities<'a>);

    fn run(&mut self, (mut vel, player, input, ent): Self::SystemData) {
        use specs::Join;
        // iterator over velocities with player components and input
        for (vel, _player, _e) in (&mut vel, &player, &ent).join() {        
            //println!("Input proc for player {}", &player.player_name);    
            let multi_axis = (input.dirs_pressed[0] && (input.dirs_pressed[2] || input.dirs_pressed[3]))
                || (input.dirs_pressed[1] && (input.dirs_pressed[2] || input.dirs_pressed[3]));
            let mut vec_amt = 80.0;
            if multi_axis {
                vec_amt = 57.0;
            }
            if input.dirs_pressed[0] {
                vel.x = -vec_amt;
            }
            else if input.dirs_pressed[1] {
                vel.x = vec_amt;
            }
            else {
                vel.x = 0.0;
            }
            if input.dirs_pressed[2] || input.jump_pressed {
                vel.y = -vec_amt;
            }
            else if input.dirs_pressed[3] {
                vel.y = vec_amt;
            }
            else {
                vel.y = 0.0;
            }
        }
    }
}

// handle ai sim input state to control Npcs
pub struct NpcInputSystem;

// handle updating actors based on velocity or controlled value
pub struct MoveActorsSys;

// handle interactions between interactive actors
pub struct InterActorSys;

// handle sorting the sprites for drawing
pub struct ZOrderSpriteSys;
