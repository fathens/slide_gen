use alignment::model::{generate_surfaces, Size3D};
use bevy::prelude::*;

pub fn run() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(setup.system());
    app.init_resource::<CubesResource>();
    app.add_system(generate_cubes.system());

    app.run();
}

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

fn setup(mut commands: Commands) {
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(-10.0, -20.0, -10.0)),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(-8.0, 8.0, 10.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}

fn generate_cubes(
    resource: ResMut<CubesResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let parts = generate_surfaces(resource.spaces);

    let calc_pos = |p: u8, s: u8| {
        let v = (p as f32) - (s as f32) / 2.0;
        resource.cube_size * v
    };

    parts.iter().for_each(|pos| {
        let tr = Vec3::new(
            calc_pos(pos.x(), resource.spaces.x()),
            calc_pos(pos.y(), resource.spaces.y()),
            calc_pos(pos.z(), resource.spaces.z()),
        );
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: resource.cube_size * 0.8,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(tr),
            ..Default::default()
        });
    });
}
