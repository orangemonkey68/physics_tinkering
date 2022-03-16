pub struct PhysicsWorld {
    gravity: Vector<f64>
    objects: Vec<PhysicsObject>
}

impl PhysicsWorld {
    fn contains(&self, obj: &PhysicsObject) -> Bool {
        &self.objects.contains(obj)
    }

    fn update(&self) {
        unimplemented()!;
    }
}

enum Shape {
    Plane,
    Sphere
}

pub trait <T: Shape> CollisionDetector<T>{
    fn check(&self, world: &PhysicsWorld);
}

pub trait <U: Shape> Collides {
    fn get_collider(&self) -> CollisionDetector<U>,
    fn get_coefficient_of_friction(&self) -> f64,
    fn get_coefficient_of_restitution(&self) -> f64
}

pub struct PhysicsObject{
    pub velocity: Vector<f64>,
    pub mass: f64,
    coefficient_of_friction: f64,
    coefficient_of_restitution: f64
}

impl Collides for PhysicsObject {
    fn get_coefficient_of_friction(&self) {
        &self.coefficient_of_friction
    }

    fn get_coefficient_of_restitution(&self) {
        &self.coefficient_of_restitution
    }
}