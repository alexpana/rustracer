use vec3::*;
use ray::*;

pub struct HitResult {
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Vec3
}

pub trait Hittable {
    fn hit(self, ray: Ray) -> Option<HitResult>;
}

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32
}

impl Hittable for Sphere {
    fn hit(self, ray: Ray) -> Option<HitResult> {
        let oc = ray.origin - self.origin;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = 2.0 * Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            let point_on_sphere = ray.point_at(t);
            return Some(HitResult {
                point: point_on_sphere,
                normal: (point_on_sphere - self.origin).unit(),
                color: V3_ZERO
            })
        }
        return None;
    }
}

pub struct Background {
    pub color_a: Vec3,
    pub color_b: Vec3
}

impl Hittable for Background {
    fn hit(self, ray: Ray) -> Option<HitResult> {
        // transform y from -1..1 to 0..1
        let t = 0.5 * (ray.direction.y + 1.0);
        // blend white to blue
        let color = blend(self.color_a, self.color_b, t);

        Some(HitResult {
            point: ray.point_at(1000.0),
            color: color,
            normal: Vec3::new(0.0, 0.0, 1.0)
        })
    }
}