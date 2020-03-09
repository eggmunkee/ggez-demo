use specs::{Builder,Entity,EntityBuilder,World,WorldExt};

use crate::components::{Position, Velocity, DisplayComp};
use crate::systems::*;

pub struct Ball;

impl Ball {
    pub fn build(world: &mut World, x: f32, y: f32, vx: f32, vy: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: vx, y: vy, gravity: true })
        .with(DisplayComp { circle: true })
        .build()
    }

    pub fn build_static(world: &mut World, x: f32, y: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(DisplayComp { circle: true })
        .build()
    }
}