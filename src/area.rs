use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{camera, util};

const WALL_HEIGHT_HALF_SIZE: f32 = 10.0;
const WALL_THICKNESS: f32 = 0.1;

pub fn setup_area(mut commands: Commands) {
    // TODO 如果两个区域的墙存在重叠，则此区域是连通的
    // 计算屏幕大小和窗口位置
    let primary_display = util::primary_display();

    // 区域大小跟相机高度有关，0.00092为手工测试出的比例
    let factor = camera::CAMERA_HEIGHT_SIZE * 0.00092;
    let area_length = factor * primary_display.width as f32;
    let area_width = factor * primary_display.height as f32;
    println!("area: {} x {}", area_length, area_width);

    // 长方体地面
    commands
        .spawn(Collider::cuboid(100000.0, 0.1, 100000.0))
        .insert(TransformBundle::from(Transform::from_translation(
            Vec3::ZERO,
        )));

    // 围墙（从Y向NEG_Y俯视）
    // 左围墙
    commands
        .spawn(Collider::cuboid(
            WALL_THICKNESS,
            WALL_HEIGHT_HALF_SIZE,
            area_width / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            -area_length / 2.0,
            WALL_HEIGHT_HALF_SIZE,
            0.0,
        )));
    // 右围墙
    commands
        .spawn(Collider::cuboid(
            WALL_THICKNESS,
            WALL_HEIGHT_HALF_SIZE,
            area_width / 2.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            area_length / 2.0,
            WALL_HEIGHT_HALF_SIZE,
            0.0,
        )));
    // 下围墙
    commands
        .spawn(Collider::cuboid(
            area_length / 2.0,
            WALL_HEIGHT_HALF_SIZE,
            WALL_THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT_HALF_SIZE,
            area_width / 2.0,
        )));
    // 上围墙
    commands
        .spawn(Collider::cuboid(
            area_length / 2.0,
            WALL_HEIGHT_HALF_SIZE,
            WALL_THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT_HALF_SIZE,
            -area_width / 2.0,
        )));
}
