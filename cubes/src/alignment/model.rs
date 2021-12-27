use num_integer::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
pub struct Cube<'a> {
    home_face: &'a Face,
    home_pos: PosAtFace,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Face {
    direction: Direction,
    limit_pos: PosAtFace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentSolid<'a> {
    members: HashMap<Face, Vec<Cube<'a>>>,
}

impl PosAtFace {
    pub fn new(h: u8, v: u8) -> PosAtFace {
        PosAtFace {
            left_right: h,
            up_down: v,
        }
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
}
