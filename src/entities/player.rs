use specs::{Builder,Entity,EntityBuilder,World,WorldExt};

use crate::components::{Position, Velocity,DisplayComp};
use crate::components::player::{PlayerComponent};
use crate::systems::*;

pub struct PlayerEntity;

impl PlayerEntity {
    pub fn build(world: &mut World, x: f32, y: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: false })
        .with(DisplayComp { circle: false })
        .with(PlayerComponent { player_name: String::from("Noah"), life: 100 })
        .build()
    }

}