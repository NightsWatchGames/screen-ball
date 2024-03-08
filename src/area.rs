use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::{camera, util};
use crate::camera::CAMERA_HEIGHT_SIZE;

const WALL_HEIGHT_SIZE: f32 = 20.0;
const WALL_THICKNESS: f32 = 0.1;

#[derive(Component)]
pub struct Wall;

pub fn setup_area(mut commands: Commands) {
    // TODO 多窗口支持：如果两个区域的墙存在重叠，则此区域是连通的
    let projection = PerspectiveProjection::default();
    let area_height = projection.fov.tan() * CAMERA_HEIGHT_SIZE * 2.0;
    let area_width = area_height * projection.aspect_ratio;
    println!("area: {} x {}", area_width, area_width);

    // 长方体地面
    commands.spawn((
        Collider::cuboid(100000.0, 0.1, 100000.0),
        RigidBody::Static,
        TransformBundle::from(Transform::from_translation(Vec3::ZERO)),
    ));

    // 围墙（从Y向NEG_Y俯视）
    // 左围墙
    commands.spawn((
        Wall,
        Collider::cuboid(WALL_THICKNESS, WALL_HEIGHT_SIZE, area_height),
        RigidBody::Static,
        TransformBundle::from(Transform::from_xyz(
            -area_width / 2.0,
            WALL_HEIGHT_SIZE / 2.0,
            0.0,
        )),
    ));
    // 右围墙
    commands.spawn((
        Wall,
        Collider::cuboid(WALL_THICKNESS, WALL_HEIGHT_SIZE, area_height),
        RigidBody::Static,
        TransformBundle::from(Transform::from_xyz(
            area_width / 2.0,
            WALL_HEIGHT_SIZE / 2.0,
            0.0,
        )),
    ));
    // 下围墙
    commands.spawn((
        Wall,
        Collider::cuboid(area_width, WALL_HEIGHT_SIZE, WALL_THICKNESS),
        RigidBody::Static,
        TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT_SIZE / 2.0,
            area_height / 2.0,
        )),
    ));
    // 上围墙
    commands.spawn((
        Wall,
        Collider::cuboid(area_width, WALL_HEIGHT_SIZE, WALL_THICKNESS),
        RigidBody::Static,
        TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT_SIZE / 2.0,
            -area_height / 2.0,
        )),
    ));
}

pub fn update_wall(q_camera: Query<&Projection>, ) {
    let proj = q_camera.single();
    let (fov, aspect_ratio) = match proj {
        Projection::Perspective(per) => {
            (per.fov, per.aspect_ratio)
        }
        Projection::Orthographic(_) => unreachable!()
    };

    let new_height = fov.tan() * CAMERA_HEIGHT_SIZE * 2.0;
    let new_width = new_height * aspect_ratio;
    println!("LWZTEST area: {} x {}", new_width, new_height);
}