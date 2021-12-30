use derive_new::new;
use getset::*;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, CopyGetters)]
pub struct Size3D {
    #[getset(get_copy = "pub")]
    x: u8,
    #[getset(get_copy = "pub")]
    y: u8,
    #[getset(get_copy = "pub")]
    z: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, CopyGetters)]
pub struct Pos3D {
    #[getset(get_copy = "pub")]
    x: u8,
    #[getset(get_copy = "pub")]
    y: u8,
    #[getset(get_copy = "pub")]
    z: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, CopyGetters)]
pub struct Cube {
    #[getset(get_copy = "pub")]
    home: Pos3D,
}

pub fn geenrate_surfaces(size: Size3D) -> Vec<Pos3D> {
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
    use std::vec;

    use super::*;

    #[test]
    fn generate() {
        let x = 5;
        let y = 4;
        let z = 3;
        let parts = geenrate_surfaces(Size3D::new(x, y, z));

        let mut all = vec![];
        (0..x).for_each(|xi| {
            (0..y).for_each(|yi| {
                (0..z).for_each(|zi| {
                    if xi == 0
                        || yi == 0
                        || zi == 0
                        || xi == (x - 1)
                        || yi == (y - 1)
                        || zi == (z - 1)
                    {
                        let p = Pos3D::new(xi, yi, zi);
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
}
