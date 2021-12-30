use alignment::model::*;
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
}
