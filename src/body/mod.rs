use crate::ray::{HitRecord, Hittable, Ray};
use serde::{Deserialize, Serialize};

mod ellipsoid;
mod sphere;

pub use ellipsoid::Ellipsoid;
pub use sphere::Sphere;
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Body {
    Sphere(Sphere),
    Ellipsoid(Ellipsoid),
}

impl Hittable for Body {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, t_min, t_max),
            Self::Ellipsoid(e) => e.hit(ray, t_min, t_max),
        }
    }
}
