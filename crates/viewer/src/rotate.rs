use crate::components::*;
use bevy::input::mouse::*;
use bevy::prelude::*;

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn action(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query_camera: Query<(&mut PanOrbitCamera, &mut Transform), Without<CubeRoot>>,
    mut query_root: Query<&mut Transform, With<CubeRoot>>,
) {
    let orbit_button = MouseButton::Left;

    let (mut pan_orbit, mut transform_camera) = query_camera.single_mut();
    let mut transform_cubes = query_root.single_mut();

    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        let up = transform_cubes.rotation * Vec3::X;
        pan_orbit.upside_down = up.x <= 0.0;
    }

    let mut scroll = 0.0;
    for ev in ev_scroll.iter() {
        scroll += ev.y / 100.0;
    }
    if scroll.abs() > 0.0 {
        pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
        // dont allow zoom to reach zero or you get stuck
        pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);

        let rot_matrix = Mat3::from_quat(transform_camera.rotation);
        transform_camera.translation =
            pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
    }

    let mut rotation_move = Vec2::ZERO;
    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move -= ev.delta;
        }
    }
    if rotation_move.length_squared() > 0.0 {
        let window = get_primary_window_size(&windows);
        let delta_x = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
        let delta_y = {
            let delta = rotation_move.y / window.y * std::f32::consts::PI;
            if pan_orbit.upside_down {
                -delta
            } else {
                delta
            }
        };
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        transform_cubes.rotation = yaw * transform_cubes.rotation; // rotate around global y axis
        transform_cubes.rotation *= pitch; // rotate around local x axis
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}

/// Spawn a camera like this
pub fn setup(mut commands: Commands) {
    let translation = Vec3::new(5.0, 5.0, 5.0);
    let radius = translation.length();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        });
}
