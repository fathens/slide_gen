use std::collections::HashMap;

use alignment::{
    model::*,
    slide::{adjacents, move_one, slide},
};
use rand::prelude::*;
use smallvec::SmallVec;

pub fn rand_hole(size: Size3D) -> Pos3D {
    let mut rng = rand::thread_rng();

    let ai = rng.gen_range(0..3);
    let ps: SmallVec<[u8; 3]> = [size.x(), size.y(), size.z()]
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            if i == ai {
                if rng.gen() {
                    0
                } else {
                    v - 1
                }
            } else {
                rng.gen_range(0..v)
            }
        })
        .collect();

    Pos3D::new(ps[0], ps[1], ps[2])
}

pub fn simple_moves(size: Size3D, steps: u8) -> HashMap<Pos3D, Cube> {
    let mut parts: HashMap<Pos3D, Cube> = generate_surfaces(size)
        .into_iter()
        .map(|pos| (pos, Cube::new(pos)))
        .collect();

    let hole = rand_hole(size);
    parts.remove(&hole);

    let mut rng = rand::thread_rng();

    (0..steps).fold(hole, |hole, _| {
        let ds = adjacents(hole, size);
        let d = ds[rng.gen_range(0..ds.len())];
        let pos = move_one(hole, size, d).unwrap_or(hole);
        slide(&mut parts, size, pos, d.invert());
        pos
    });

    parts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assert_rand_hole() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let size = Size3D::new(
                rng.gen_range(3..10),
                rng.gen_range(3..10),
                rng.gen_range(3..10),
            );
            let pos = rand_hole(size);
            println!("{:?}: {:?}", size, pos);
            assert!(pos.on_face(size));
        }
    }

    #[test]
    fn hole_of_simple_moves() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let size = Size3D::new(
                rng.gen_range(3..10),
                rng.gen_range(3..10),
                rng.gen_range(3..10),
            );
            let parts = simple_moves(size, rng.gen_range(10..100));

            let mut holes = vec![];
            generate_surfaces(size).into_iter().for_each(|pos| {
                if !parts.contains_key(&pos) {
                    holes.push(pos);
                }
            });

            assert_eq!(1, holes.len());
            assert!(holes[0].on_face(size));
        }
    }
}
