use specs::{Component, DenseVecStorage, World, WorldExt};
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
    fn pt_block_check(&self, check_point: &Point2<f32>) -> (bool, f32);
    fn pt_vector_check(&self, check_point: &Point2<f32>, vector: &Vector2<f32>) -> bool;
}

impl Collidable for Collision {
    // that point is within block o
    fn pt_block_check(&self, check_point: &Point2<f32>) -> (bool, f32) {
        let pt = Point2::new(500.0f32,500.0);
        let radius = 300.0f32;
        let d = distance(&pt, check_point);
        //if d < radius {
            //println!("Block passed for (0,0) and {:?}", check_point);
        //}

        (d < radius, d)
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

