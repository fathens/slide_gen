use std::collections::HashMap;

use crate::model::{Cube, Direction, Pos3D, Size3D};

pub fn move_one(pos: Pos3D, size: Size3D, d: Direction) -> Option<Pos3D> {
    match d {
        Direction::XNega => {
            if pos.x() == 0 {
                None
            } else {
                Some(Pos3D::new(pos.x() - 1, pos.y(), pos.z()))
            }
        }
        Direction::XPosi => {
            if pos.x() == size.x() - 1 {
                None
            } else {
                Some(Pos3D::new(pos.x() + 1, pos.y(), pos.z()))
            }
        }
        Direction::YNega => {
            if pos.y() == 0 {
                None
            } else {
                Some(Pos3D::new(pos.x(), pos.y() - 1, pos.z()))
            }
        }
        Direction::YPosi => {
            if pos.y() == size.y() - 1 {
                None
            } else {
                Some(Pos3D::new(pos.x(), pos.y() + 1, pos.z()))
            }
        }
        Direction::ZNega => {
            if pos.z() == 0 {
                None
            } else {
                Some(Pos3D::new(pos.x(), pos.y(), pos.z() - 1))
            }
        }
        Direction::ZPosi => {
            if pos.z() == size.z() - 1 {
                None
            } else {
                Some(Pos3D::new(pos.x(), pos.y(), pos.z() + 1))
            }
        }
    }
}

pub fn slide(parts: &mut HashMap<Pos3D, Cube>, size: Size3D, target: Pos3D, d: Direction) -> bool {
    match move_one(target, size, d) {
        None => false,
        Some(next_pos) => {
            if !next_pos.on_face(size) || parts.contains_key(&next_pos) {
                false
            } else {
                match parts.remove(&target) {
                    None => false,
                    Some(cube) => {
                        parts.insert(next_pos, cube);
                        true
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::geenrate_surfaces;

    use super::*;

    #[test]
    fn pos_move_one() {
        assert_eq!(
            move_one(Pos3D::new(0, 4, 6), Size3D::new(3, 5, 7), Direction::XNega),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction::XNega),
            Some(Pos3D::new(1, 4, 6))
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction::XPosi),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(1, 4, 6), Size3D::new(3, 5, 7), Direction::XPosi),
            Some(Pos3D::new(2, 4, 6))
        );

        assert_eq!(
            move_one(Pos3D::new(2, 0, 6), Size3D::new(3, 5, 7), Direction::YNega),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction::YNega),
            Some(Pos3D::new(2, 3, 6))
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction::YPosi),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 3, 6), Size3D::new(3, 5, 7), Direction::YPosi),
            Some(Pos3D::new(2, 4, 6))
        );

        assert_eq!(
            move_one(Pos3D::new(2, 4, 0), Size3D::new(3, 5, 7), Direction::ZNega),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction::ZNega),
            Some(Pos3D::new(2, 4, 5))
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction::ZPosi),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 5), Size3D::new(3, 5, 7), Direction::ZPosi),
            Some(Pos3D::new(2, 4, 6))
        );
    }

    #[test]
    fn slides() {
        let size = Size3D::new(3, 4, 5);
        let mut parts: HashMap<Pos3D, Cube> = geenrate_surfaces(size)
            .into_iter()
            .map(|pos| (pos, Cube::new(pos)))
            .collect();

        let pos234 = Pos3D::new(2, 3, 4);
        parts.remove(&pos234);

        let pos224 = Pos3D::new(2, 2, 4);
        let cube224 = *parts.get(&pos224).unwrap();
        assert!(!slide(&mut parts, size, pos224, Direction::XNega));
        assert!(!slide(&mut parts, size, pos224, Direction::XPosi));
        assert!(!slide(&mut parts, size, pos224, Direction::ZNega));
        assert!(!slide(&mut parts, size, pos224, Direction::ZPosi));
        assert!(slide(&mut parts, size, pos224, Direction::YPosi));
        assert!(!slide(&mut parts, size, pos224, Direction::YNega));
        assert_eq!(*parts.get(&pos234).unwrap(), cube224);

        let pos223 = Pos3D::new(2, 2, 3);
        let cube223 = *parts.get(&pos223).unwrap();
        assert!(!slide(&mut parts, size, pos223, Direction::XPosi));
        assert!(!slide(&mut parts, size, pos223, Direction::XNega));
        assert!(!slide(&mut parts, size, pos223, Direction::YPosi));
        assert!(!slide(&mut parts, size, pos223, Direction::YNega));
        assert!(slide(&mut parts, size, pos223, Direction::ZPosi));
        assert!(!slide(&mut parts, size, pos223, Direction::ZNega));
        assert_eq!(*parts.get(&pos224).unwrap(), cube223);

        let pos323 = Pos3D::new(3, 2, 3);
        assert_eq!(parts.get(&pos323), None);
        assert!(!slide(&mut parts, size, pos323, Direction::XPosi));
        assert!(!slide(&mut parts, size, pos323, Direction::XNega));
        assert!(!slide(&mut parts, size, pos323, Direction::YPosi));
        assert!(!slide(&mut parts, size, pos323, Direction::YNega));
        assert!(!slide(&mut parts, size, pos323, Direction::ZPosi));
        assert!(!slide(&mut parts, size, pos323, Direction::ZNega));
    }
}
