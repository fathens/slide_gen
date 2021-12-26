use num_integer::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    XPosi,
    XNega,
    YPosi,
    YNega,
    ZPosi,
    ZNega,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosAtFace {
    left_right: u8,
    up_down: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cube<'a> {
    home_face: &'a Face,
    home_pos: PosAtFace,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Face {
    direction: Direction,
    limit_pos: PosAtFace,
}

#[derive(Clone, PartialEq, Eq)]
pub struct CurrentSolid<'a> {
    members: HashMap<Face, Vec<Cube<'a>>>,
}

impl Face {
    pub fn get_index(&self, pos: PosAtFace) -> u8 {
        pos.up_down * self.limit_pos.left_right + pos.left_right
    }

    pub fn get_pos(&self, index: u8) -> PosAtFace {
        let (v, h) = index.div_rem(&self.limit_pos.left_right);
        PosAtFace {
            left_right: h,
            up_down: v,
        }
    }
}
