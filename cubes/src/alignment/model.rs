use num_integer::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction {
    XPosi,
    XNega,
    YPosi,
    YNega,
    ZPosi,
    ZNega,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosAtFace {
    left_right: u8,
    up_down: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square {
    home_direction: Direction,
    home_pos: PosAtFace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Face {
    direction: Direction,
    limit_pos: PosAtFace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentFaces {
    faces: HashMap<Face, Vec<Square>>,
}

impl PosAtFace {
    pub fn new(h: u8, v: u8) -> PosAtFace {
        PosAtFace {
            left_right: h,
            up_down: v,
        }
    }
}

impl Square {
    pub fn new(d: Direction, pos: PosAtFace) -> Square {
        Square {
            home_direction: d,
            home_pos: pos,
        }
    }

    pub fn get_direction(&self) -> Direction {
        self.home_direction
    }

    pub fn get_pos(&self) -> PosAtFace {
        self.home_pos
    }
}

impl Face {
    pub fn new(d: Direction, size: PosAtFace) -> Face {
        Face {
            direction: d,
            limit_pos: size,
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn size(&self) -> PosAtFace {
        self.limit_pos
    }

    fn generate_cubes(&self) -> Vec<Square> {
        let max = self.limit_pos.left_right * self.limit_pos.up_down;
        (0..max)
            .map(|i| Square::new(self.direction, self.get_pos(i)))
            .collect()
    }

    /**
    Convert index to position.

    # Example
    ```
    use crate::cubes::alignment::model::*;
    let face = Face::new(Direction::XNega, PosAtFace::new(5, 7));
    let pos = PosAtFace::new(1, 2);
    assert_eq!(face.get_index(pos), 11);
    ```
     */
    pub fn get_index(&self, pos: PosAtFace) -> u8 {
        pos.up_down * self.limit_pos.left_right + pos.left_right
    }

    /**
    Convert position to index.

    # Example
    ```
    use crate::cubes::alignment::model::*;
    let face = Face::new(Direction::XNega, PosAtFace::new(5, 7));
    assert_eq!(face.get_pos(11), PosAtFace::new(1, 2));
    ```
     */
    pub fn get_pos(&self, index: u8) -> PosAtFace {
        let (v, h) = index.div_rem(&self.limit_pos.left_right);
        PosAtFace::new(h, v)
    }
}

impl CurrentFaces {
    pub fn geenrate<F>(f: F) -> CurrentFaces
    where
        F: Fn(Direction) -> PosAtFace,
    {
        let faces = Direction::iter()
            .map(|d| {
                let face = Face::new(d, f(d));
                (face, face.generate_cubes())
            })
            .collect();
        CurrentFaces { faces }
    }

    pub fn get_by_direction(&self, d: Direction) -> &Vec<Square> {
        self.faces
            .iter()
            .find(|(face, _)| face.direction == d)
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_index_and_pos() {
        let face = Face::new(Direction::XNega, PosAtFace::new(5, 7));

        // over `left_right`
        assert_eq!(face.get_index(PosAtFace::new(8, 2)), 18);
        assert_eq!(face.get_pos(18), PosAtFace::new(3, 3));

        // over `up_down`
        assert_eq!(face.get_index(PosAtFace::new(2, 9)), 47);
        assert_eq!(face.get_pos(47), PosAtFace::new(2, 9));

        // over both
        assert_eq!(face.get_index(PosAtFace::new(8, 9)), 53);
        assert_eq!(face.get_pos(53), PosAtFace::new(3, 10));
    }

    #[test]
    fn generate_faces() {
        let faces = CurrentFaces::geenrate(|d| match d {
            Direction::YNega => PosAtFace::new(4, 4),
            _ => PosAtFace::new(5, 5),
        });
        assert_eq!(16, faces.get_by_direction(Direction::YNega).len());
        assert_eq!(25, faces.get_by_direction(Direction::XPosi).len());
    }
}
