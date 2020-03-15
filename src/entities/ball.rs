use ggez::{Context};
use specs::{Builder,Entity,EntityBuilder,World,WorldExt};

use crate::components::{Position, Velocity, DisplayComp, DisplayCompType};
use crate::components::ball::*;
use crate::components::collision::{Collision};

pub struct BallBuilder;

impl BallBuilder {
    pub fn build(world: &mut World, ctx: &mut Context, x: f32, y: f32, vx: f32, vy: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: vx, y: vy, gravity: true })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(BallDisplayComponent::new(ctx, &"/green-spotted-circle.png".to_string()))
        .with(Collision::new_circle(20.0))
        .build()
    }

    pub fn build_static(world: &mut World, ctx: &mut Context, x: f32, y: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(BallDisplayComponent::new(ctx, &"/green-spotted-circle.png".to_string()))
        .with(Collision::new_circle(20.0))
        .build()
    }
}