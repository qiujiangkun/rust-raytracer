use serde::{Deserialize, Serialize};

use crate::materials::Material;
use crate::point3d::Point3D;
use crate::ray::HitRecord;
use crate::ray::Hittable;
use crate::ray::Ray;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ellipsoid {
    pub center: Point3D,
    pub radii: Point3D, // Semi-axes lengths along x, y, and z axes
    pub material: Material,
}

impl Ellipsoid {
    pub fn new(center: Point3D, radii: Point3D, material: Material) -> Ellipsoid {
        Ellipsoid {
            center,
            radii,
            material,
        }
    }
}

fn u_v_from_ellipsoid_hit_point(hit_point_on_ellipsoid: Point3D, radii: Point3D) -> (f64, f64) {
    let n = hit_point_on_ellipsoid / radii; // Normalize the hit point by the radii
    let x = n.x();
    let y = n.y();
    let z = n.z();
    let u = (x.atan2(z) / (2.0 * std::f64::consts::PI)) + 0.5;
    let v = y * 0.5 + 0.5;
    (u, v)
}

impl Hittable for Ellipsoid {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = (ray.direction / self.radii).length_squared();
        let half_b = oc.dot(ray.direction / self.radii);
        let c = oc.length_squared() / self.radii.length_squared() - 1.0; // Element-wise division
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < t_max && *root > t_min {
                    let p = ray.at(*root);
                    let normal = (p - self.center) / (self.radii * self.radii); // Normalize by the squared radii
                    let front_face = ray.direction.dot(normal) < 0.0;

                    let (u, v) = u_v_from_ellipsoid_hit_point(p - self.center, self.radii);

                    return Some(HitRecord {
                        t: *root,
                        point: p,
                        normal: if front_face { normal } else { -normal },
                        front_face,
                        material: &self.material,
                        u,
                        v,
                    });
                }
            }
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use crate::materials::Glass;
    #[cfg(test)]
    use crate::materials::Lambertian;
    #[cfg(test)]
    use palette::Srgb;

    #[test]
    fn test_ellipsoid_hit() {
        let center = Point3D::new(0.0, 0.0, 0.0);
        let radii = Point3D::new(1.0, 1.0, 2.0);
        let ellipsoid = Ellipsoid::new(center, radii, Material::Glass(Glass::new(1.5)));
        let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Point3D::new(0.0, 0.0, 1.0));
        let hit = ellipsoid.hit(&ray, 0.0, f64::INFINITY);
        assert_eq!(hit.unwrap().t, 0.6547694874158765);
    }

    #[test]
    fn test_ellipsoid_texture_coordinates() {
        let center = Point3D::new(0.0, 0.0, 0.0);
        let radii = Point3D::new(1.0, 1.0, 1.0);
        let ellipsoid = Ellipsoid::new(
            center,
            radii,
            Material::Lambertian(Lambertian::new(Srgb::new(0.5, 0.5, 0.5))),
        );

        // Assuming you have a method to check the texture coordinates within a small epsilon
        fn check_texture_coordinates(hit: Option<HitRecord>, expected_u: f64, expected_v: f64) {
            let epsilon = 1e-6;
            let hit_record = hit.unwrap();
            assert!((hit_record.u - expected_u).abs() < epsilon);
            assert!((hit_record.v - expected_v).abs() < epsilon);
        }

        // Test ellipsoid with radii (1, 1, 1)
        let ray = Ray::new(Point3D::new(0.0, 0.0, -1.0), Point3D::new(0.0, 0.0, 1.0));
        let hit = ellipsoid.hit(&ray, 0.0, f64::INFINITY);
        check_texture_coordinates(hit, 0.5, 0.5);

        // Test ellipsoid with radii (1, 2, 3)
        let radii = Point3D::new(1.0, 2.0, 3.0);
        let ellipsoid = Ellipsoid::new(
            center,
            radii,
            Material::Lambertian(Lambertian::new(Srgb::new(0.5, 0.5, 0.5))),
        );
        let hit = ellipsoid.hit(&ray, 0.0, f64::INFINITY);
        check_texture_coordinates(hit, 0.5, 0.5);
    }
}
