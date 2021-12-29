use alignment::model::*;
use rand::prelude::*;

pub fn rand_hole(x: u8, y: u8, z: u8) -> Pos3D {
    let mut rng = rand::thread_rng();

    let ai = rng.gen_range(0..3);
    let ps: Vec<_> = [x, y, z]
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            if i == ai {
                if rng.gen() {
                    0
                } else {
                    v
                }
            } else {
                rng.gen_range(0..v)
            }
        })
        .collect();

    Pos3D::new(ps[0], ps[1], ps[2])
}
