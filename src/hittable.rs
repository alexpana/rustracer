use vec3::{blend, Vec3};
use vec3;
use ray::*;

use std::f32;

pub struct HitResult {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Vec3
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitResult>;
}

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub color: Vec3
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitResult> {
        let oc = ray.origin - self.origin;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = 2.0 * Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;
        if d >= 0.0 {
            let t = if d.sqrt() < -b {
                (-b - d.sqrt()) / (2.0 * a)
            } else {
                (-b + d.sqrt()) / (2.0 * a)
            };

            if tmin <= t && t <= tmax {
                let point_on_sphere = ray.point_at(t);
                return Some(HitResult {
                    t: t,
                    point: point_on_sphere,
                    normal: (point_on_sphere - self.origin).unit(),
                    color: self.color
                })
            }
        }
        return None;
    }
}

pub struct Background {
    pub color_from: Vec3,
    pub color_to: Vec3
}

impl Hittable for Background {
    fn hit(&self, ray: Ray, _: f32, tmax: f32) -> Option<HitResult> {
        let t = 0.5 * (ray.direction.y + 1.0);

        let color = blend(self.color_from, self.color_to, t);

        Some(HitResult {
            t: tmax - 1.0,
            point: ray.point_at(tmax - 1.0),
            color: color,
            normal: color * 2.0 - vec3::ONE
        })
    }
}

pub struct Scene {
    pub objects: Vec<Box<Hittable>>,
    pub background: Background
}

impl Hittable for Scene {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32) -> Option<HitResult> {
        let mut closest_hit_distance = tmax;
        let mut closest_hit = Option::None;

        for object in self.objects.iter() {
            match (*object).hit(ray, tmin, tmax) {
                Some(hit) => {
                    if hit.t <= closest_hit_distance {
                        closest_hit_distance = hit.t;
                        closest_hit = Some(hit);
                    }
                }
                None => ()
            }
        }

        closest_hit
    }
}
