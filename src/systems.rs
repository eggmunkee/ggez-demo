
use specs::{
    Builder, DispatcherBuilder,
    Component, Entities,
    ReadStorage, WriteStorage, System, VecStorage, 
    World, WorldExt, RunNow
};
use specs::shred::{Dispatcher};

use crate::components::{Position,Velocity,GridLoc};

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
        let mut i = 0;
        for (vel, pos) in (&mut vel, &mut pos).join() {
            // update positions by velocity vectors
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
            // Constrain position to a box (hardcoded atm)
            self::UpdatePos::box_pos(pos, vel);
            //println!("UpdatePos on #{}", &i);
            i += 1;
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
pub struct GameCommandSystem;
// handle changes to the game state
pub struct GameStateSys;

// handle input state to control Players
pub struct InputSystem {
    dir_press: [u16;4]
}
impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem {
            dir_press: [0,0,0,0].into()
        }
    }
}
impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadStorage<'a, Velocity>,
                       ReadStorage<'a, Position>,
                        Entities<'a>);

    fn run(&mut self, (vel, pos, ent): Self::SystemData) {
        use specs::Join;
        let mut i = 0;
        for (vel, pos, e) in (&vel, &pos, &ent).join() {            
            //println!("[INPUT SYSTEM] Ent {:?}, Pos: {:?} Vel: {:?}", &e, &pos, &vel);
            i += 1;
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
