use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size3D {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos3D {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cube {
    home: Pos3D,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cubes {
    size: Size3D,
    parts: HashMap<Cube, Pos3D>,
}

impl Size3D {
    pub fn new(x: u8, y: u8, z: u8) -> Size3D {
        Size3D { x, y, z }
    }
}

impl Pos3D {
    pub fn new(x: u8, y: u8, z: u8) -> Pos3D {
        Pos3D { x, y, z }
    }
}

impl Cube {
    pub fn new(home: Pos3D) -> Cube {
        Cube { home }
    }
}

impl Cubes {
    pub fn geenrate(x: u8, y: u8, z: u8) -> Cubes {
        let mut parts = HashMap::new();
        (0..y).for_each(|yi| {
            (0..z).for_each(|zi| {
                let home = Pos3D::new(0, yi, zi);
                parts.insert(Cube::new(home), home);
            });
        });
        (0..y).for_each(|yi| {
            (0..z).for_each(|zi| {
                let home = Pos3D::new(x - 1, yi, zi);
                parts.insert(Cube::new(home), home);
            });
        });
        (1..(x - 1)).for_each(|xi| {
            (0..z).for_each(|zi| {
                let home = Pos3D::new(xi, 0, zi);
                parts.insert(Cube::new(home), home);
            })
        });
        (1..(x - 1)).for_each(|xi| {
            (0..z).for_each(|zi| {
                let home = Pos3D::new(xi, y - 1, zi);
                parts.insert(Cube::new(home), home);
            })
        });
        (1..(y - 1)).for_each(|yi| {
            (1..(x - 1)).for_each(|xi| {
                let home = Pos3D::new(xi, yi, 0);
                parts.insert(Cube::new(home), home);
            })
        });
        (1..(y - 1)).for_each(|yi| {
            (1..(x - 1)).for_each(|xi| {
                let home = Pos3D::new(xi, yi, z - 1);
                parts.insert(Cube::new(home), home);
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
        let keys: Vec<_> = stage.parts.keys().collect();
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
                        let cs: Vec<_> = keys.iter().filter(|c| c.home == p).collect();
                        assert_eq!(1, cs.len());
                        all.push(p);
                    }
                })
            })
        });

        assert_eq!(stage.parts.len(), all.len());
    }
}
