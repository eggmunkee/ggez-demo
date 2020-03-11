use specs::{Builder, Component, DispatcherBuilder, ReadStorage, WriteStorage, System, DenseVecStorage, World, WorldExt, RunNow};
//use specs::shred::{Dispatcher};

#[derive(Debug)]
pub enum CollisionShape {
    Circle(f32),
    Square(f32)
}

#[derive(Debug)]
pub struct Collision {
    pub shape: CollisionShape,
}

impl Collision {
    pub fn new() -> Collision {
        Collision {
            shape: CollisionShape::Circle(32.0)
        }
    }
    pub fn new_circle(radius: f32) -> Collision {
        Collision {
            shape: CollisionShape::Circle(radius)
        }
    }
    pub fn new_square(radius: f32) -> Collision {
        Collision {
            shape: CollisionShape::Square(radius)
        }
    }
}

pub trait Collidable {

}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}



// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<Collision>();
}

