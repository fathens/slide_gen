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

        let mut mate: StandardMaterial = Color::rgba(1.0, 1.0, 1.0, 0.4).into();
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

                    let image = draw_image(100, resource.spaces, home, direction, reversed);

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

fn draw_image(
    image_size: u8,
    spaces: Size3D,
    pos: Pos3D,
    direction: Direction3D,
    reversed: bool,
) -> Image {
    let image_width = image_size as f32;
    let image_height = image_size as f32;

    let (width, height) = match direction {
        Direction3D::XNega => (spaces.z(), spaces.y()),
        Direction3D::XPosi => (spaces.z(), spaces.y()),
        Direction3D::YNega => (spaces.z(), spaces.x()),
        Direction3D::YPosi => (spaces.z(), spaces.x()),
        Direction3D::ZNega => (spaces.y(), spaces.x()),
        Direction3D::ZPosi => (spaces.y(), spaces.x()),
    };

    let (pos_x, pos_y) = match direction {
        Direction3D::XNega => (pos.z(), pos.y()),
        Direction3D::XPosi => (pos.z(), pos.y()),
        Direction3D::YNega => (pos.z(), pos.x()),
        Direction3D::YPosi => (pos.z(), pos.x()),
        Direction3D::ZNega => (pos.y(), pos.x()),
        Direction3D::ZPosi => (pos.y(), pos.x()),
    };

    let (sig_x, sig_y) = match direction {
        Direction3D::XNega => (false, !reversed),
        Direction3D::XPosi => (false, reversed),
        Direction3D::YNega => (false, reversed),
        Direction3D::YPosi => (false, !reversed),
        Direction3D::ZNega => (reversed, true),
        Direction3D::ZPosi => (!reversed, true),
    };

    let base_color = match direction {
        Direction3D::XNega => Color::RED,
        Direction3D::XPosi => Color::CYAN,
        Direction3D::YNega => Color::GREEN,
        Direction3D::YPosi => Color::PURPLE,
        Direction3D::ZNega => Color::BLUE,
        Direction3D::ZPosi => Color::YELLOW,
    };

    let mut image_data = Vec::with_capacity((image_width * image_height) as usize * 4);
    let mut put_color = |c: Color| {
        [c.r(), c.g(), c.b(), c.a()].into_iter().for_each(|v| {
            image_data.push((v * 255.0) as u8);
        });
    };

    let r_units = image_width.max(image_height) / 2.0;
    let center_x = image_width * width as f32 / 2.0;
    let center_y = image_height * height as f32 / 2.0;

    let normalize = |size: f32, pos: u8, p: u8, sig: bool| -> f32 {
        let base = size * pos as f32;
        if sig {
            base + p as f32
        } else {
            base + (size - p as f32 - 1.0)
        }
    };

    for x in 0..(image_width as u8) {
        for y in 0..(image_height as u8) {
            let pixel_x = normalize(image_width, pos_x, x, sig_x);
            let pixel_y = normalize(image_height, pos_y, y, sig_y);
            let d_x = pixel_x - center_x;
            let d_y = pixel_y - center_y;
            let d = (d_x.powf(2.0) + d_y.powf(2.0)).sqrt();
            let u = (d / r_units) as u8;
            if u % 2 == 1 {
                put_color(base_color);
            } else {
                put_color(Color::WHITE);
            }
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
