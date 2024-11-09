use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryMonitor, PrimaryWindow};
// 相机在y轴高度
pub const CAMERA_HEIGHT_SIZE: f32 = 10.0;

pub fn setup_primary_window(
    mut primary_window: Single<&mut Window, With<PrimaryWindow>>,
    primary_monitor: Single<&Monitor, With<PrimaryMonitor>>,
) {
    // windows系统上窗口不能跟显示器一样大，会导致背景不是透明（为黑色）
    primary_window.resolution.set_physical_resolution(
        primary_monitor.physical_width - 1,
        primary_monitor.physical_height - 1,
    );
}

pub fn setup_camera(
    mut commands: Commands,
    primary_monitor: Single<&Monitor, With<PrimaryMonitor>>,
    q_other_monitors: Query<&Monitor, Without<PrimaryMonitor>>,
) {
    // light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, CAMERA_HEIGHT_SIZE, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
    ));

    println!("primary monitor: {:?}", primary_monitor.into_inner());
    for monitor in q_other_monitors.iter() {
        println!("other monitor: {:?}", monitor)
    }

    // TODO 多窗口支持：根据display信息创建新窗口
}
