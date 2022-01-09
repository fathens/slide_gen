use crate::resources::CubesResource;
use alignment::model::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubeHome(Pos3D);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubePos(Pos3D);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubeFace(Direction3D);

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
        .for_each(|home| {
            let center = Vec3::new(
                calc_pos(home.x(), resource.spaces.x()),
                calc_pos(home.y(), resource.spaces.y()),
                calc_pos(home.z(), resource.spaces.z()),
            );

            /* TODO Add a transparent cube body
            commands
                .spawn()
                .insert(CubeHome(home))
                .insert(CubePos(home))
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube {
                        size: resource.cube_size * 0.9,
                    })),
                    material: materials.add(Color::rgba(0.0, 150.0, 200.0, 0.1).into()),
                    transform: Transform::from_translation(center),
                    ..Default::default()
                });
            */

            let face_size = resource.cube_size * 0.94;
            let face_half = 1.01 * face_size / 2.0;

            home.get_faces(resource.spaces)
                .into_iter()
                .for_each(|direction| {
                    let tr = match direction {
                        Direction3D::XNega => Transform {
                            translation: center + Vec3::new(-face_half, 0.0, 0.0),
                            rotation: Quat::from_rotation_z(90_f32.to_radians()),
                            ..Default::default()
                        },
                        Direction3D::XPosi => Transform {
                            translation: Vec3::new(face_half, 0.0, 0.0),
                            rotation: Quat::from_rotation_z(-90_f32.to_radians()),
                            ..Default::default()
                        },
                        Direction3D::YNega => Transform {
                            translation: center + Vec3::new(0.0, -face_half, 0.0),
                            rotation: Quat::from_rotation_z(180_f32.to_radians()),
                            ..Default::default()
                        },
                        Direction3D::YPosi => Transform {
                            translation: center + Vec3::new(0.0, face_half, 0.0),
                            ..Default::default()
                        },
                        Direction3D::ZNega => Transform {
                            translation: center + Vec3::new(0.0, 0.0, -face_half),
                            rotation: Quat::from_rotation_x(-90_f32.to_radians()),
                            ..Default::default()
                        },
                        Direction3D::ZPosi => Transform {
                            translation: center + Vec3::new(0.0, 0.0, face_half),
                            rotation: Quat::from_rotation_x(90_f32.to_radians()),
                            ..Default::default()
                        },
                    };

                    commands
                        .spawn()
                        .insert(CubeHome(home))
                        .insert(CubePos(home))
                        .insert(CubeFace(direction))
                        .insert_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Plane { size: face_size })),
                            material: materials.add(Color::rgb(200.0, 100.0, 0.0).into()),
                            transform: tr,
                            ..Default::default()
                        });
                });
        });
}
