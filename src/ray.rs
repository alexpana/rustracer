use vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin: origin, direction: direction.unit() }
    }

    pub fn point_at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use Ray;
    use vec3::Vec3;

    #[test]
    fn point_at() {
        let ray = Ray::new(Vec3::new2d(0.0, 0.0), Vec3::new2d(1.0, 2.0));
        assert_eq! (ray.point_at(2.5), Vec3::new2d(1.0, 2.0).unit() * 2.5)
    }
}