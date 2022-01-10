use crate::components::*;
use crate::resources::CubesResource;
use alignment::model::*;
use bevy::prelude::*;
use shuffle::rand_hole;

pub fn setup(
    resource: ResMut<CubesResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hole = rand_hole(resource.spaces);
    let cubes = generate_surfaces(resource.spaces)
        .into_iter()
        .filter(|home| *home != hole);

    commands.spawn().insert(CubeHole(hole));

    cubes.into_iter().for_each(|home| {
        let center = resource.calc_center(home);

        /* TODO Add a transparent cube body */
        commands
            .spawn()
            .insert(CubeHome(home))
            .insert(CubePos(home))
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: resource.cube_size * 0.9,
                })),
                material: materials.add(Color::rgb(0.0, 0.9, 0.7).into()),
                transform: Transform::from_translation(center),
                ..Default::default()
            });

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
                        translation: center + Vec3::new(face_half, 0.0, 0.0),
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
                        material: materials.add(Color::rgb(1.0, 0.5, 0.0).into()),
                        transform: tr,
                        ..Default::default()
                    });
            });
    });
}
