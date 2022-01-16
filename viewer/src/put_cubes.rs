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

    let body_size = resource.cube_size * 0.95;
    let face_size = resource.cube_size * 0.96;
    let face_half = face_size / 2.0;

    commands.spawn().insert(CubeHole(hole));

    cubes.into_iter().for_each(|home| {
        let center = resource.calc_center(home);

        let mut mate: StandardMaterial = Color::rgba(0.0, 0.9, 0.7, 0.4).into();
        mate.alpha_mode = AlphaMode::Blend;
        commands
            .spawn()
            .insert(CubeHome(home))
            .insert(CubePos(home))
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: body_size })),
                material: materials.add(mate),
                transform: Transform::from_translation(center),
                ..Default::default()
            });

        home.get_faces(resource.spaces)
            .into_iter()
            .for_each(|direction| {
                [true, false].into_iter().for_each(|reversed| {
                    let mut tr = mk_transform(direction, reversed);
                    tr.translation *= face_half;
                    tr.translation += center;

                    let image = draw_image(100, 100);

                    commands
                        .spawn()
                        .insert(CubeHome(home))
                        .insert(CubePos(home))
                        .insert(CubeFace(direction))
                        .insert_bundle(PbrBundle {
                            mesh: meshes.add(shape::Plane { size: face_size }.into()),
                            material: materials.add(images.add(image).into()),
                            transform: tr,
                            ..Default::default()
                        });
                });
            });
    });
}

fn mk_transform(direction: Direction3D, reversed: bool) -> Transform {
    let rev: f32 = if reversed { 180.0 } else { 0.0 };
    match direction {
        Direction3D::XNega => Transform {
            translation: Vec3::new(-1.0, 0.0, 0.0),
            rotation: Quat::from_rotation_z((rev + 90.0).to_radians()),
            ..Default::default()
        },
        Direction3D::XPosi => Transform {
            translation: Vec3::new(1.0, 0.0, 0.0),
            rotation: Quat::from_rotation_z((rev - 90.0).to_radians()),
            ..Default::default()
        },
        Direction3D::YNega => Transform {
            translation: Vec3::new(0.0, -1.0, 0.0),
            rotation: Quat::from_rotation_z((rev + 180.0).to_radians()),
            ..Default::default()
        },
        Direction3D::YPosi => Transform {
            translation: Vec3::new(0.0, 1.0, 0.0),
            rotation: Quat::from_rotation_z(rev.to_radians()),
            ..Default::default()
        },
        Direction3D::ZNega => Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            rotation: Quat::from_rotation_x((rev - 90.0).to_radians()),
            ..Default::default()
        },
        Direction3D::ZPosi => Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            rotation: Quat::from_rotation_x((rev + 90.0).to_radians()),
            ..Default::default()
        },
    }
}

fn draw_image(image_width: usize, image_height: usize) -> Image {
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

    image
}
