use crate::components::*;
use crate::resources::CubesResource;
use alignment::model::*;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use shuffle::rand_hole;

pub fn setup(
    resource: ResMut<CubesResource>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let hole = rand_hole(resource.spaces);
    let cubes = generate_surfaces(resource.spaces)
        .into_iter()
        .filter(|home| *home != hole);

    let face_size = resource.cube_size * 0.94;
    let face_half = 1.01 * face_size / 2.0;

    commands.spawn().insert(CubeHole(hole));

    cubes.into_iter().for_each(|home| {
        let center = resource.calc_center(home);

        let mut mate: StandardMaterial = Color::rgba(0.0, 0.9, 0.7, 0.8).into();
        mate.alpha_mode = AlphaMode::Blend;
        commands
            .spawn()
            .insert(CubeHome(home))
            .insert(CubePos(home))
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: resource.cube_size * 0.9,
                })),
                material: materials.add(mate),
                transform: Transform::from_translation(center),
                ..Default::default()
            });

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

                let image_width = 100;
                let image_height = 100;

                // TODO draw image
                let mut image_data = Vec::with_capacity(image_width * image_height * 4);
                for x in 0..image_width {
                    for y in 0..image_height {
                        let r = (x as f32 / image_width as f32) * 255.0;
                        let g = (y as f32 / image_height as f32) * 255.0;
                        let b = (r + g) / 2.0;
                        image_data.push(r as u8);
                        image_data.push(g as u8);
                        image_data.push(b as u8);
                        image_data.push(255);
                    }
                }

                let image = Image::new(
                    Extent3d {
                        width: image_width as u32,
                        height: image_height as u32,
                        ..Default::default()
                    },
                    TextureDimension::D2,
                    image_data,
                    TextureFormat::Rgba8Unorm,
                );

                commands
                    .spawn()
                    .insert(CubeHome(home))
                    .insert(CubePos(home))
                    .insert(CubeFace(direction))
                    .insert_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Plane { size: face_size })),
                        material: materials.add(images.add(image).into()),
                        transform: tr,
                        ..Default::default()
                    });
            });
    });
}
