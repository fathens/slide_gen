use crate::resources::CubesResource;
use alignment::model::{generate_surfaces, Pos3D};
use bevy::prelude::*;

pub struct CubeId(Pos3D);

impl CubeId {
    pub fn from(pos: Pos3D) -> CubeId {
        // let mut hasher = DefaultHasher::new();
        // pos.hash(&mut hasher);
        // let hash = hasher.finish();
        CubeId(pos)
    }
}

#[derive(Bundle)]
pub struct CubePart {
    home: Pos3D,
    #[bundle]
    body: PbrBundle,
}

pub fn generate_cubes(
    resource: ResMut<CubesResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let calc_pos = |p: u8, s: u8| {
        let v = (p as f32) - (s as f32) / 2.0;
        resource.cube_size * v
    };

    generate_surfaces(resource.spaces).into_iter().for_each(|pos| {
        let tr = Vec3::new(
            calc_pos(pos.x(), resource.spaces.x()),
            calc_pos(pos.y(), resource.spaces.y()),
            calc_pos(pos.z(), resource.spaces.z()),
        );
        let body = PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: resource.cube_size * 0.9,
            })),
            material: materials.add(Color::rgba(0.0, 0.6, 1.0, 0.5).into()),
            transform: Transform::from_translation(tr),
            ..Default::default()
        };

        commands
            .spawn()
            .insert(CubeId::from(pos))
            .insert_bundle(CubePart { home: pos, body });
    });
}
