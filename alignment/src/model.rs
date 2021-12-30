use derive_new::new;
use getset::*;
use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, Eq, Getters, CopyGetters, MutGetters)]
pub struct Cubes {
    #[getset(get_copy = "pub")]
    size: Size3D,
    #[getset(get = "pub", get_mut = "pub(crate)")]
    parts: HashMap<Pos3D, Cube>,
}

impl Cubes {
    pub fn geenrate(x: u8, y: u8, z: u8) -> Cubes {
        let mut parts = HashMap::new();
        let mut set_cube = |home: Pos3D| {
            parts.insert(home, Cube::new(home));
        };

        (0..y).for_each(|yi| {
            (0..z).for_each(|zi| {
                set_cube(Pos3D::new(0, yi, zi));
            });
        });
        (0..y).for_each(|yi| {
            (0..z).for_each(|zi| {
                set_cube(Pos3D::new(x - 1, yi, zi));
            });
        });
        (1..(x - 1)).for_each(|xi| {
            (0..z).for_each(|zi| {
                set_cube(Pos3D::new(xi, 0, zi));
            })
        });
        (1..(x - 1)).for_each(|xi| {
            (0..z).for_each(|zi| {
                set_cube(Pos3D::new(xi, y - 1, zi));
            })
        });
        (1..(y - 1)).for_each(|yi| {
            (1..(x - 1)).for_each(|xi| {
                set_cube(Pos3D::new(xi, yi, 0));
            })
        });
        (1..(y - 1)).for_each(|yi| {
            (1..(x - 1)).for_each(|xi| {
                set_cube(Pos3D::new(xi, yi, z - 1));
            })
        });

        Cubes {
            size: Size3D::new(x, y, z),
            parts,
        }
    }
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
        let stage = Cubes::geenrate(x, y, z);

        let mut all = vec![];
        let keys: Vec<_> = stage.parts.values().collect();
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
                        let cs: Vec<_> = keys.iter().filter(|c| c.home() == p).collect();
                        assert_eq!(1, cs.len());
                        all.push(p);
                    }
                })
            })
        });

        assert_eq!(stage.parts.len(), all.len());
    }
}
