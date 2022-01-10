use alignment::model::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubeHome(pub Pos3D);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubePos(pub Pos3D);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubeHole(pub Pos3D);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubeFace(pub Direction3D);
