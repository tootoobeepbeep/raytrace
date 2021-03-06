use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {

    pub fn point_at_t(&self, t: f64) -> Vec3 {
        self.origin + (self.direction.scalar_mult(t))
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn new(origin:Vec3, direction:Vec3) -> Self {
        Ray{origin, direction}
    }


}
