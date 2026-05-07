use std::ops::Index;

pub const MIN: f32 = f32::MIN;
pub const MAX: f32 = f32::MAX;

type Point3 = nalgebra::Point3<f32>;
type Vector3 = nalgebra::Vector3<f32>;

#[derive(Debug, PartialEq, Clone)]
pub struct Bounds3 {
    pub p_min: Point3,
    pub p_max: Point3
}

impl Bounds3 {
    pub fn new() -> Self {
        Self {
            p_min: Point3::new(MAX, MAX, MAX),
            p_max: Point3::new(MIN, MIN, MIN)
        }
    }

    pub fn init_one(p: &Point3) -> Self {
        Self {
            p_min: p.clone(),
            p_max: p.clone()
        }
    }

    pub fn init_two(p1: &Point3, p2: &Point3) -> Self {
        Self {
            p_min: Point3::new(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z)),
            p_max: Point3::new(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z)),
        }
    }

    pub fn diagonal(&self) -> Vector3 {
        self.p_max - self.p_min
    }

    pub fn corner(&self, corner: usize) -> Point3 {
        let x_idx = corner & 1;
        let y_idx = if (corner & 2) != 0 { 1 } else { 0 };
        let z_idx = if (corner & 4) != 0 { 1 } else { 0 };

        let x = self[x_idx].x;
        let y = self[y_idx].y;
        let z = self[z_idx].z;

        Point3::new(x, y, z)
    }

    pub fn volume(&self) -> f32 {
        let d = self.diagonal();

        d.x * d.y * d.z
    }

    pub fn surface_area(&self) -> f32 {
        let d = self.diagonal();

        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    // Returns which extent is longer
    pub fn max_extent(&self) -> usize {
        let d = self.diagonal();

        if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }

    pub fn offset(&self, p: &Point3) -> Point3 {
        let mut o: Point3 = (p - self.p_min).into();

        if self.p_max.x > self.p_min.x {
            o.x /= self.p_max.x - self.p_min.x;
        }

        if self.p_max.y > self.p_min.y {
            o.y /= self.p_max.y - self.p_min.y;
        }

        if self.p_max.z > self.p_min.z {
            o.z /= self.p_max.z - self.p_min.z;
        }

        o
    }

    pub fn bounding_sphere(&self, c: &mut Point3, r: &mut f32) {
        *c = (self.p_min -  (-1.0) * self.p_max).into();
        *c /= 2.0;

        *r = if self.inside(&c) {
            (self.p_max - *c).norm()
        } else {
            0.0
        };
    }

    pub fn inside(&self, p: &Point3) -> bool {
        self.p_min.x < p.x && p.x < self.p_max.x &&
        self.p_min.y < p.y && p.y < self.p_max.y &&
        self.p_min.z < p.z && p.z < self.p_max.z
    }

    pub fn inside_exclusive(&self, p: &Point3) -> bool {
        p.x >= self.p_min.x && p.x < self.p_max.x &&
        p.y >= self.p_min.y && p.y < self.p_max.y &&
        p.z >= self.p_min.z && p.z < self.p_max.z
    }

    pub fn union_p(&self, p: &Point3) -> Self {
        let p_minx = self.p_min.x.min(p.x);
        let p_miny = self.p_min.y.min(p.y);
        let p_minz = self.p_min.z.min(p.z);

        let p_maxx = self.p_max.x.max(p.x);
        let p_maxy = self.p_max.y.max(p.y);
        let p_maxz = self.p_max.z.max(p.z);

        Self {
            p_min: Point3::new(p_minx, p_miny, p_minz),
            p_max: Point3::new(p_maxx, p_maxy, p_maxz),
        }
    }

    pub fn union(&self, b: &Bounds3) -> Self {
        let p_minx = self.p_min.x.min(b.p_min.x);
        let p_miny = self.p_min.y.min(b.p_min.y);
        let p_minz = self.p_min.z.min(b.p_min.z);

        let p_maxx = self.p_max.x.max(b.p_max.x);
        let p_maxy = self.p_max.y.max(b.p_max.y);
        let p_maxz = self.p_max.z.max(b.p_max.z);

        Self {
            p_min: Point3::new(p_minx, p_miny, p_minz),
            p_max: Point3::new(p_maxx, p_maxy, p_maxz),
        }
    }

    pub fn intersect(&self, b: &Bounds3) -> Self {
        let p_minx = self.p_min.x.max(b.p_min.x);
        let p_miny = self.p_min.y.max(b.p_min.y);
        let p_minz = self.p_min.z.max(b.p_min.z);

        let p_maxx = self.p_max.x.min(b.p_max.x);
        let p_maxy = self.p_max.y.min(b.p_max.y);
        let p_maxz = self.p_max.z.min(b.p_max.z);

        Self {
            p_min: Point3::new(p_minx, p_miny, p_minz),
            p_max: Point3::new(p_maxx, p_maxy, p_maxz),
        }
    }

    pub fn overlaps(&self, b: Bounds3) -> bool {
        let x = self.p_max.x >= b.p_min.x && self.p_min.x <= b.p_max.x;
        let y = self.p_max.y >= b.p_min.y && self.p_min.y <= b.p_max.y;
        let z = self.p_max.z >= b.p_min.z && self.p_min.z <= b.p_max.z;

        x && y && z
    }

    pub fn expand(&self, delta: f32) -> Self {
        let pmin = self.p_min - Vector3::new(delta, delta, delta);
        let pmax = self.p_max + Vector3::new(delta, delta, delta);

        Self::init_two(&pmin, &pmax)
    }
}


impl Index<usize> for Bounds3 {
    type Output = Point3;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index == 0 || index == 1);

        if index == 0 {
            &self.p_min
        } else {
            &self.p_max
        }
    }
}
