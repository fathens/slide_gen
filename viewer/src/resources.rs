use alignment::model::Size3D;
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
