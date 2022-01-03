use crate::resources::CubesResource;
use alignment::{
    model,
    model::{generate_surfaces, Pos3D},
};
use bevy::prelude::*;

pub struct CubeId {
    home: Pos3D,
}

#[derive(Bundle)]
pub struct CubeBody {
    pos: Pos3D,
    #[bundle]
    body: PbrBundle,
}

#[derive(Bundle)]
pub struct CubeFace {
    direction: model::Direction,
    #[bundle]
    face: PbrBundle,
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

    generate_surfaces(resource.spaces)
        .into_iter()
        .for_each(|pos| {
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

            let face_size = resource.cube_size * 0.94;
            let face_half = face_size / 2.0;

            let mut entity = commands.spawn();
            entity.insert(CubeId { home: pos });
            pos.get_faces(resource.spaces)
                .into_iter()
                .for_each(|direction| {
                    let tr = match direction {
                        _ => Transform {
                            translation: tr + Vec3::new(-face_half, 0.0, 0.0),
                            rotation: Quat::from_rotation_z(90_f32.to_radians()),
                            ..Default::default()
                        },
                        // model::Direction::XPosi =>Transform {
                        //     translation: Vec3::new(half, 0.0, 0.0),
                        //     rotation: Quat::from_rotation_z(-90_f32.to_radians()),
                        //     ..Default::default()
                        // }
                    };
                    let face = PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Plane { size: face_size })),
                        material: materials.add(Color::rgb(1.0, 0.6, 0.0).into()),
                        transform: tr,
                        ..Default::default()
                    };
                    entity.insert_bundle(CubeFace { direction, face });
                });
            entity.insert_bundle(CubeBody { pos, body });
        });
}
