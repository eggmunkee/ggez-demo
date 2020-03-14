use specs::{Builder, Component, DispatcherBuilder, ReadStorage, WriteStorage, System, DenseVecStorage, World, WorldExt, RunNow};
//use specs::shred::{Dispatcher};
use ggez::nalgebra::{Point2,Vector2,distance};

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
    #[allow(dead_code)]
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
    fn pt_block_check(&self, check_point: &Point2<f32>) -> bool;
    fn pt_vector_check(&self, check_point: &Point2<f32>, vector: &Vector2<f32>) -> bool;
}

impl Collidable for Collision {
    fn pt_block_check(&self, check_point: &Point2<f32>) -> bool {
        let pt = Point2::new(0.0f32,0.0);
        let d = distance(&pt, check_point);
        if (d < 55.0) {
            println!("Block passed for (0,0) and {:?}", check_point);
        }
        d < 55.0
    }
    fn pt_vector_check(&self, check_point: &Point2<f32>, vector: &Vector2<f32>) -> bool {
        true
    }
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}



// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<Collision>();
}

