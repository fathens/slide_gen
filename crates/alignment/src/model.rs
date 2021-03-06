use derive_new::new;
use getset::*;
use std::{hash::Hash, vec};
use strum_macros::EnumIter;
use tinyvec::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, new, CopyGetters)]
pub struct Size3D {
    #[getset(get_copy = "pub")]
    x: u8,
    #[getset(get_copy = "pub")]
    y: u8,
    #[getset(get_copy = "pub")]
    z: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction3D {
    XNega,
    XPosi,
    YNega,
    YPosi,
    ZNega,
    ZPosi,
}

impl Default for Direction3D {
    fn default() -> Self {
        Direction3D::XNega
    }
}

impl Direction3D {
    #[must_use]
    pub fn invert(self) -> Self {
        match self {
            Direction3D::XNega => Direction3D::XPosi,
            Direction3D::XPosi => Direction3D::XNega,
            Direction3D::YNega => Direction3D::YPosi,
            Direction3D::YPosi => Direction3D::YNega,
            Direction3D::ZNega => Direction3D::ZPosi,
            Direction3D::ZPosi => Direction3D::ZNega,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, new, CopyGetters)]
pub struct Pos3D {
    #[getset(get_copy = "pub")]
    x: u8,
    #[getset(get_copy = "pub")]
    y: u8,
    #[getset(get_copy = "pub")]
    z: u8,
}

impl Pos3D {
    pub fn on_face(self, size: Size3D) -> bool {
        !self.get_faces(size).is_empty()
    }

    pub fn get_faces(self, size: Size3D) -> ArrayVec<[Direction3D; 3]> {
        let mut results = array_vec!([Direction3D; 3]);
        if size.x <= self.x || size.y <= self.y || size.z <= self.z {
            return results;
        }
        if self.x == 0 {
            results.push(Direction3D::XNega)
        }
        if self.y == 0 {
            results.push(Direction3D::YNega)
        }
        if self.z == 0 {
            results.push(Direction3D::ZNega)
        }
        if self.x == size.x - 1 {
            results.push(Direction3D::XPosi)
        }
        if self.y == size.y - 1 {
            results.push(Direction3D::YPosi)
        }
        if self.z == size.z - 1 {
            results.push(Direction3D::ZPosi)
        }
        results
    }

    pub fn distance(self, other: Pos3D) -> f32 {
        let diff = |a, b| ((a as f32) - (b as f32)).powi(2);
        let x = diff(self.x, other.x);
        let y = diff(self.y, other.y);
        let z = diff(self.z, other.z);
        (x + y + z).sqrt()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, CopyGetters)]
pub struct Cube {
    #[getset(get_copy = "pub")]
    home: Pos3D,
}

pub fn generate_surfaces(size: Size3D) -> Vec<Pos3D> {
    let mut parts = vec![];
    let mut set_cube = |xi, yi, zi| {
        parts.push(Pos3D::new(xi, yi, zi));
    };

    (0..size.y).for_each(|yi| {
        (0..size.z).for_each(|zi| {
            set_cube(0, yi, zi);
        });
    });
    (0..size.y).for_each(|yi| {
        (0..size.z).for_each(|zi| {
            set_cube(size.x - 1, yi, zi);
        });
    });
    (1..(size.x - 1)).for_each(|xi| {
        (0..size.z).for_each(|zi| {
            set_cube(xi, 0, zi);
        })
    });
    (1..(size.x - 1)).for_each(|xi| {
        (0..size.z).for_each(|zi| {
            set_cube(xi, size.y - 1, zi);
        })
    });
    (1..(size.y - 1)).for_each(|yi| {
        (1..(size.x - 1)).for_each(|xi| {
            set_cube(xi, yi, 0);
        })
    });
    (1..(size.y - 1)).for_each(|yi| {
        (1..(size.x - 1)).for_each(|xi| {
            set_cube(xi, yi, size.z - 1);
        })
    });

    parts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate() {
        let size = Size3D::new(5, 4, 3);
        let parts = generate_surfaces(size);

        let mut all = vec![];
        (0..size.x).for_each(|xi| {
            (0..size.y).for_each(|yi| {
                (0..size.z).for_each(|zi| {
                    let p = Pos3D::new(xi, yi, zi);
                    if p.on_face(size) {
                        println!("{:?}", p);
                        let cs: Vec<_> = parts.iter().filter(|c| **c == p).collect();
                        assert_eq!(1, cs.len());
                        all.push(p);
                    }
                })
            })
        });

        assert_eq!(parts.len(), all.len());
    }

    #[test]
    fn pos_on_face() {
        assert!(Pos3D::new(2, 4, 6).on_face(Size3D::new(3, 5, 7)));
        assert!(Pos3D::new(1, 4, 6).on_face(Size3D::new(3, 5, 7)));
        assert!(Pos3D::new(1, 3, 6).on_face(Size3D::new(3, 5, 7)));
        assert!(!Pos3D::new(1, 3, 5).on_face(Size3D::new(3, 5, 7)));
        assert!(!Pos3D::new(3, 4, 6).on_face(Size3D::new(3, 5, 7)));
        assert!(!Pos3D::new(2, 5, 6).on_face(Size3D::new(3, 5, 7)));
        assert!(!Pos3D::new(2, 4, 7).on_face(Size3D::new(3, 5, 7)));
    }
}
