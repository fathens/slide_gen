use smallvec::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::model::{Cube, Direction3D, Pos3D, Size3D};

pub fn move_one(pos: Pos3D, size: Size3D, d: Direction3D) -> Option<Pos3D> {
    match d {
        Direction3D::XNega => (pos.x() > 0).then(|| Pos3D::new(pos.x() - 1, pos.y(), pos.z())),
        Direction3D::XPosi => {
            (pos.x() < size.x() - 1).then(|| Pos3D::new(pos.x() + 1, pos.y(), pos.z()))
        }
        Direction3D::YNega => (pos.y() > 0).then(|| Pos3D::new(pos.x(), pos.y() - 1, pos.z())),
        Direction3D::YPosi => {
            (pos.y() < size.y() - 1).then(|| Pos3D::new(pos.x(), pos.y() + 1, pos.z()))
        }
        Direction3D::ZNega => (pos.z() > 0).then(|| Pos3D::new(pos.x(), pos.y(), pos.z() - 1)),
        Direction3D::ZPosi => {
            (pos.z() < size.z() - 1).then(|| Pos3D::new(pos.x(), pos.y(), pos.z() + 1))
        }
    }
}

pub fn slide(parts: &mut HashMap<Pos3D, Cube>, size: Size3D, src: Pos3D, d: Direction3D) -> bool {
    match move_one(src, size, d) {
        None => false,
        Some(next_pos) => {
            if !next_pos.on_face(size) || parts.contains_key(&next_pos) {
                false
            } else {
                match parts.remove(&src) {
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

pub fn adjacents(center: Pos3D, size: Size3D) -> SmallVec<[Direction3D; 4]> {
    let mut results = smallvec![];
    Direction3D::iter().for_each(|d| {
        if let Some(pos) = move_one(center, size, d) {
            if pos.on_face(size) {
                results.push(d);
            }
        }
    });
    results
}

#[cfg(test)]
mod test {
    use crate::model::generate_surfaces;

    use super::*;

    #[test]
    fn pos_move_one() {
        assert_eq!(
            move_one(Pos3D::new(0, 4, 6), Size3D::new(3, 5, 7), Direction3D::XNega),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction3D::XNega),
            Some(Pos3D::new(1, 4, 6))
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction3D::XPosi),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(1, 4, 6), Size3D::new(3, 5, 7), Direction3D::XPosi),
            Some(Pos3D::new(2, 4, 6))
        );

        assert_eq!(
            move_one(Pos3D::new(2, 0, 6), Size3D::new(3, 5, 7), Direction3D::YNega),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction3D::YNega),
            Some(Pos3D::new(2, 3, 6))
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction3D::YPosi),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 3, 6), Size3D::new(3, 5, 7), Direction3D::YPosi),
            Some(Pos3D::new(2, 4, 6))
        );

        assert_eq!(
            move_one(Pos3D::new(2, 4, 0), Size3D::new(3, 5, 7), Direction3D::ZNega),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction3D::ZNega),
            Some(Pos3D::new(2, 4, 5))
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 6), Size3D::new(3, 5, 7), Direction3D::ZPosi),
            None
        );
        assert_eq!(
            move_one(Pos3D::new(2, 4, 5), Size3D::new(3, 5, 7), Direction3D::ZPosi),
            Some(Pos3D::new(2, 4, 6))
        );
    }

    #[test]
    fn slides() {
        let size = Size3D::new(3, 4, 5);
        let mut parts: HashMap<Pos3D, Cube> = generate_surfaces(size)
            .into_iter()
            .map(|pos| (pos, Cube::new(pos)))
            .collect();

        let pos234 = Pos3D::new(2, 3, 4);
        assert!(parts.remove(&pos234).is_some());

        let pos224 = Pos3D::new(2, 2, 4);
        let cube224 = *parts.get(&pos224).unwrap();
        assert!(!slide(&mut parts, size, pos224, Direction3D::XNega));
        assert!(!slide(&mut parts, size, pos224, Direction3D::XPosi));
        assert!(!slide(&mut parts, size, pos224, Direction3D::ZNega));
        assert!(!slide(&mut parts, size, pos224, Direction3D::ZPosi));
        assert!(slide(&mut parts, size, pos224, Direction3D::YPosi));
        assert!(!slide(&mut parts, size, pos224, Direction3D::YNega));
        assert_eq!(*parts.get(&pos234).unwrap(), cube224);

        let pos223 = Pos3D::new(2, 2, 3);
        let cube223 = *parts.get(&pos223).unwrap();
        assert!(!slide(&mut parts, size, pos223, Direction3D::XNega));
        assert!(!slide(&mut parts, size, pos223, Direction3D::XPosi));
        assert!(!slide(&mut parts, size, pos223, Direction3D::YNega));
        assert!(!slide(&mut parts, size, pos223, Direction3D::YPosi));
        assert!(!slide(&mut parts, size, pos223, Direction3D::ZNega));
        assert!(slide(&mut parts, size, pos223, Direction3D::ZPosi));
        assert_eq!(*parts.get(&pos224).unwrap(), cube223);

        assert_eq!(parts.get(&pos223), None);
        assert!(!slide(&mut parts, size, pos223, Direction3D::XNega));
        assert!(!slide(&mut parts, size, pos223, Direction3D::XPosi));
        assert!(!slide(&mut parts, size, pos223, Direction3D::YNega));
        assert!(!slide(&mut parts, size, pos223, Direction3D::YPosi));
        assert!(!slide(&mut parts, size, pos223, Direction3D::ZNega));
        assert!(!slide(&mut parts, size, pos223, Direction3D::ZPosi));

        let pos323 = Pos3D::new(3, 2, 3);
        assert_eq!(parts.get(&pos323), None);
        assert!(!slide(&mut parts, size, pos323, Direction3D::XNega));
        assert!(!slide(&mut parts, size, pos323, Direction3D::XPosi));
        assert!(!slide(&mut parts, size, pos323, Direction3D::YNega));
        assert!(!slide(&mut parts, size, pos323, Direction3D::YPosi));
        assert!(!slide(&mut parts, size, pos323, Direction3D::ZNega));
        assert!(!slide(&mut parts, size, pos323, Direction3D::ZPosi));
    }

    #[test]
    fn slides_two_holes() {
        let size = Size3D::new(3, 4, 5);
        let mut parts: HashMap<Pos3D, Cube> = generate_surfaces(size)
            .into_iter()
            .map(|pos| (pos, Cube::new(pos)))
            .collect();

        let pos000 = Pos3D::new(0, 0, 0);
        let pos001 = Pos3D::new(0, 0, 1);
        assert!(parts.remove(&pos000).is_some());
        assert!(parts.remove(&pos001).is_some());

        let pos100 = Pos3D::new(1, 0, 0);
        let cube100 = *parts.get(&pos100).unwrap();
        assert!(slide(&mut parts, size, pos100, Direction3D::XNega));
        assert!(!slide(&mut parts, size, pos100, Direction3D::XPosi));
        assert!(!slide(&mut parts, size, pos100, Direction3D::ZNega));
        assert!(!slide(&mut parts, size, pos100, Direction3D::ZPosi));
        assert!(!slide(&mut parts, size, pos100, Direction3D::YPosi));
        assert!(!slide(&mut parts, size, pos100, Direction3D::YNega));
        assert_eq!(*parts.get(&pos000).unwrap(), cube100);

        let pos101 = Pos3D::new(1, 0, 1);
        let cube101 = *parts.get(&pos101).unwrap();
        assert!(slide(&mut parts, size, pos101, Direction3D::ZNega));
        assert!(!slide(&mut parts, size, pos101, Direction3D::ZPosi));
        assert!(!slide(&mut parts, size, pos101, Direction3D::XNega));
        assert!(!slide(&mut parts, size, pos101, Direction3D::XPosi));
        assert!(!slide(&mut parts, size, pos101, Direction3D::YNega));
        assert!(!slide(&mut parts, size, pos101, Direction3D::YPosi));
        assert_eq!(*parts.get(&pos100).unwrap(), cube101);

        assert_eq!(parts.get(&pos001), None);
        assert!(!slide(&mut parts, size, pos001, Direction3D::XNega));
        assert!(!slide(&mut parts, size, pos001, Direction3D::XPosi));
        assert!(!slide(&mut parts, size, pos001, Direction3D::YNega));
        assert!(!slide(&mut parts, size, pos001, Direction3D::YPosi));
        assert!(!slide(&mut parts, size, pos001, Direction3D::ZNega));
        assert!(!slide(&mut parts, size, pos001, Direction3D::ZPosi));

        assert_eq!(parts.get(&pos101), None);
        assert!(!slide(&mut parts, size, pos101, Direction3D::XNega));
        assert!(!slide(&mut parts, size, pos101, Direction3D::XPosi));
        assert!(!slide(&mut parts, size, pos101, Direction3D::YNega));
        assert!(!slide(&mut parts, size, pos101, Direction3D::YPosi));
        assert!(!slide(&mut parts, size, pos101, Direction3D::ZNega));
        assert!(!slide(&mut parts, size, pos101, Direction3D::ZPosi));
    }

    #[test]
    fn invert_direction() {
        let size = Size3D::new(3, 3, 3);

        let check = |center, ds: SmallVec<[Direction3D; 4]>| {
            let ads = adjacents(center, size);
            println!("{:?}", ads);
            assert_eq!(ds.len(), ads.len());
            for d in ds {
                assert!(ads.contains(&d));
                let next = move_one(center, size, d).unwrap();
                assert_eq!(center, move_one(next, size, d.invert()).unwrap());
            }
        };

        check(
            Pos3D::new(1, 0, 1),
            smallvec![
                Direction3D::XNega,
                Direction3D::XPosi,
                Direction3D::ZNega,
                Direction3D::ZPosi
            ],
        );

        check(
            Pos3D::new(1, 1, 0),
            smallvec![
                Direction3D::XNega,
                Direction3D::XPosi,
                Direction3D::YNega,
                Direction3D::YPosi
            ],
        );

        check(
            Pos3D::new(0, 1, 1),
            smallvec![
                Direction3D::ZNega,
                Direction3D::ZPosi,
                Direction3D::YNega,
                Direction3D::YPosi
            ],
        );

        check(
            Pos3D::new(0, 0, 0),
            smallvec![Direction3D::XPosi, Direction3D::YPosi, Direction3D::ZPosi],
        );

        check(
            Pos3D::new(2, 0, 0),
            smallvec![Direction3D::XNega, Direction3D::YPosi, Direction3D::ZPosi],
        );

        check(
            Pos3D::new(0, 2, 0),
            smallvec![Direction3D::XPosi, Direction3D::YNega, Direction3D::ZPosi],
        );

        check(
            Pos3D::new(0, 0, 2),
            smallvec![Direction3D::XPosi, Direction3D::YPosi, Direction3D::ZNega],
        );

        check(
            Pos3D::new(2, 2, 2),
            smallvec![Direction3D::XNega, Direction3D::YNega, Direction3D::ZNega],
        );

        check(
            Pos3D::new(0, 2, 2),
            smallvec![Direction3D::XPosi, Direction3D::YNega, Direction3D::ZNega],
        );

        check(
            Pos3D::new(2, 0, 2),
            smallvec![Direction3D::XNega, Direction3D::YPosi, Direction3D::ZNega],
        );

        check(
            Pos3D::new(2, 2, 0),
            smallvec![Direction3D::XNega, Direction3D::YNega, Direction3D::ZPosi],
        );

        check(
            Pos3D::new(1, 0, 0),
            smallvec![
                Direction3D::YPosi,
                Direction3D::ZPosi,
                Direction3D::XNega,
                Direction3D::XPosi
            ],
        );

        check(
            Pos3D::new(0, 1, 0),
            smallvec![
                Direction3D::XPosi,
                Direction3D::ZPosi,
                Direction3D::YNega,
                Direction3D::YPosi
            ],
        );

        check(
            Pos3D::new(0, 0, 1),
            smallvec![
                Direction3D::XPosi,
                Direction3D::YPosi,
                Direction3D::ZNega,
                Direction3D::ZPosi
            ],
        );

        check(
            Pos3D::new(1, 2, 2),
            smallvec![
                Direction3D::ZNega,
                Direction3D::YNega,
                Direction3D::XNega,
                Direction3D::XPosi
            ],
        );

        check(
            Pos3D::new(2, 1, 2),
            smallvec![
                Direction3D::ZNega,
                Direction3D::XNega,
                Direction3D::YNega,
                Direction3D::YPosi
            ],
        );

        check(
            Pos3D::new(2, 2, 1),
            smallvec![
                Direction3D::XNega,
                Direction3D::YNega,
                Direction3D::ZNega,
                Direction3D::ZPosi
            ],
        );
    }
}
