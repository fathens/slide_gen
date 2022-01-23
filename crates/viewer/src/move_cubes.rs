use crate::components::*;
use crate::resources::CubesResource;
use alignment::model::*;
use alignment::slide::*;
use bevy::prelude::*;
use rand::prelude::*;
use tinyvec::ArrayVec;

#[derive(Debug, Clone, Component)]
pub struct ShuffleTickTimer(pub Timer);

pub struct MovingCube {
    pub home: Pos3D,
    pub prev_pos: Pos3D,
    pub next_pos: Pos3D,
    pub direction: Direction3D,
    pub step: u8,
}

impl MovingCube {
    const FASE_STEPS: u8 = 10;
}

impl Default for MovingCube {
    fn default() -> Self {
        Self {
            home: Default::default(),
            prev_pos: Default::default(),
            next_pos: Default::default(),
            direction: Direction3D::XNega,
            step: 0,
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(ShuffleTickTimer(Timer::from_seconds(0.1, true)));
}

pub fn action(
    resource: Res<CubesResource>,
    mut moving: Local<MovingCube>,
    time: Res<Time>,
    mut query_timer: Query<&mut ShuffleTickTimer>,
    mut query_hole: Query<&mut CubeHole>,
    mut query_bodies: Query<
        (&CubeHome, &mut CubePos, &mut Transform, &mut Handle<Mesh>),
        Without<CubeFace>,
    >,
) {
    let mut my_timer = query_timer.single_mut();
    if my_timer.0.tick(time.delta()).just_finished() {
        if moving.step >= MovingCube::FASE_STEPS {
            moving.step = 0;
        }

        if moving.step == 0 {
            let mut hole = query_hole.single_mut();
            info!("Current hole: {:?}", hole);
            let (next_hole, d) = shuffle_one(resource.spaces, hole.0, moving.direction);
            moving.prev_pos = next_hole;
            moving.next_pos = hole.0;
            moving.direction = d;
            moving.home = query_bodies
                .iter()
                .find(|(_, p, _, _)| p.0 == next_hole)
                .map(|(h, _, _, _)| h.0)
                .unwrap_or(next_hole);
            hole.0 = next_hole;
        }
        moving.step += 1;

        let a = resource.calc_center(moving.prev_pos);
        let b = resource.calc_center(moving.next_pos);
        let delta = (b - a) / (MovingCube::FASE_STEPS as f32);
        info!(
            "Move delta: {:?} = {:?}({:?}) - {:?}({:?})",
            delta, b, moving.next_pos, a, moving.prev_pos
        );

        for (home, mut pos, mut tr, _) in query_bodies.iter_mut() {
            if home.0 == moving.home {
                pos.0 = moving.next_pos;
                tr.translation += delta;
            }
        }
    }
}

fn shuffle_one(size: Size3D, hole: Pos3D, prev_direction: Direction3D) -> (Pos3D, Direction3D) {
    let mut rng = rand::thread_rng();

    let ds: ArrayVec<[Direction3D; 4]> = adjacents(hole, size)
        .into_iter()
        .filter(|d| *d != prev_direction)
        .collect();
    let d = ds[rng.gen_range(0..ds.len())];
    let pos = move_one(hole, size, d).unwrap_or(hole);
    (pos, d.invert())
}
