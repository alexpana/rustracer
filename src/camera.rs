use ray::Ray;
use vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left: Vec3,
    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        return Ray {
            origin: self.origin,
            direction: self.lower_left +
                Vec3::new2d(u * self.lower_left.x.abs() * 2.0 / self.width,
                            v * self.lower_left.y.abs() * 2.0 / self.height)
                - self.origin
        }
    }
}