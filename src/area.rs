use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryMonitor};

use crate::camera::CAMERA_HEIGHT_SIZE;

const WALL_HEIGHT_SIZE: f32 = 20.0;
const WALL_THICKNESS: f32 = 0.1;

pub fn setup_ground(mut commands: Commands) {
    // 长方体地面
    commands.spawn((
        Collider::cuboid(100000.0, 0.1, 100000.0),
        RigidBody::Static,
        Transform::from_translation(Vec3::ZERO),
    ));
}

pub fn setup_wall(mut commands: Commands, primary_monitor: Single<&Monitor, With<PrimaryMonitor>>) {
    // TODO 多窗口支持：如果两个区域的墙存在重叠，则此区域是连通的
    let projection = PerspectiveProjection::default();

    let area_height = projection.fov.tan() * CAMERA_HEIGHT_SIZE;
    let area_width = area_height
        * (primary_monitor.physical_width as f32 / primary_monitor.physical_height as f32);
    println!("Setup area: {} x {}", area_width, area_height);

    // 围墙（从Y向NEG_Y俯视）
    // 左围墙
    commands.spawn((
        Collider::cuboid(WALL_THICKNESS, WALL_HEIGHT_SIZE, area_height),
        RigidBody::Static,
        Transform::from_xyz(-area_width / 2.0, WALL_HEIGHT_SIZE / 2.0, 0.0),
    ));
    // 右围墙
    commands.spawn((
        Collider::cuboid(WALL_THICKNESS, WALL_HEIGHT_SIZE, area_height),
        RigidBody::Static,
        Transform::from_xyz(area_width / 2.0, WALL_HEIGHT_SIZE / 2.0, 0.0),
    ));
    // 上围墙
    commands.spawn((
        Collider::cuboid(area_width, WALL_HEIGHT_SIZE, WALL_THICKNESS),
        RigidBody::Static,
        Transform::from_xyz(0.0, WALL_HEIGHT_SIZE / 2.0, -area_height / 2.0),
    ));
    // 下围墙
    commands.spawn((
        Collider::cuboid(area_width, WALL_HEIGHT_SIZE, WALL_THICKNESS),
        RigidBody::Static,
        Transform::from_xyz(0.0, WALL_HEIGHT_SIZE / 2.0, area_height / 2.0),
    ));
}
