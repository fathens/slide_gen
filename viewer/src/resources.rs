use alignment::model::{Pos3D, Size3D};
use bevy::prelude::*;

pub struct CubesResource {
    pub spaces: Size3D,
    pub cube_size: f32,
}

impl FromWorld for CubesResource {
    fn from_world(_: &mut World) -> Self {
        CubesResource {
            spaces: Size3D::new(3, 4, 5),
            cube_size: 1.0,
        }
    }
}

impl CubesResource {
    pub fn calc_center(&self, pos: Pos3D) -> Vec3 {
        let calc_pos = |p: u8, s: u8| {
            let v = (p as f32) - (s as f32) / 2.0;
            self.cube_size * v
        };
        Vec3::new(
            calc_pos(pos.x(), self.spaces.x()),
            calc_pos(pos.y(), self.spaces.y()),
            calc_pos(pos.z(), self.spaces.z()),
        )
    }
}
