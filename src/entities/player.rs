use specs::{Builder,Entity,EntityBuilder,World,WorldExt};

use crate::components::{Position, Velocity,DisplayComp};
use crate::components::collision::{Collision};
use crate::components::player::{PlayerComponent};
use crate::systems::*;

pub struct PlayerEntity;

impl PlayerEntity {
    pub fn build(world: &mut World, x: f32, y: f32) -> Entity {
        let mut player_comp = PlayerComponent::new();
        player_comp.player_name.clear();
        player_comp.player_name.push_str("Noah");
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: false })
        .with(DisplayComp { circle: false })
        .with(Collision::new_square(5.0))
        .with(player_comp)
        .build()
    }

}