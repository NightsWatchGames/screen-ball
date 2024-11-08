use crate::camera;
use bevy::window::{Monitor, PrimaryMonitor, PrimaryWindow};
use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{CompositeAlphaMode, WindowLevel, WindowRef, WindowResolution},
};
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
    // let non_primary_displays = util::non_primary_displays();
    // for display in non_primary_displays {
    //     let window_entity = commands
    //         .spawn(Window {
    //             transparent: true,
    //             decorations: false,
    //             window_level: WindowLevel::AlwaysOnTop,
    //             #[cfg(target_os = "macos")]
    //             composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
    //             resolution: WindowResolution::new(
    //                 display.width as f32 * 0.99,
    //                 display.height as f32 * 0.99,
    //             ),
    //             ..default()
    //         })
    //         .id();

    //     // 主相机相对于窗口原点坐标
    //     let primary_camera_based_on_window_origin = Vec2::new(
    //         primary_display.width as f32 / 2.0,
    //         primary_display.height as f32 / 2.0,
    //     );
    //     dbg!(primary_camera_based_on_window_origin);
    //     // 当前相机相对于窗口原点坐标
    //     let current_camera_based_on_window_origin =
    //         Vec2::new(display.width as f32 / 2.0, display.height as f32 / 2.0)
    //             + Vec2::new(display.x as f32, display.y as f32);
    //     dbg!(current_camera_based_on_window_origin);
    //     // 当前相机相对于世界原点（即主相机位置）坐标
    //     let current_camera_based_on_world_origin =
    //         current_camera_based_on_window_origin - primary_camera_based_on_window_origin;
    //     dbg!(current_camera_based_on_world_origin);

    //     // TODO 窗口坐标单位不等同于世界坐标单位
    //     commands.spawn(Camera3dBundle {
    //         transform: Transform::from_xyz(
    //             current_camera_based_on_world_origin.x,
    //             camera::CAMERA_HEIGHT_SIZE,
    //             current_camera_based_on_world_origin.y,
    //         )
    //         .looking_at(
    //             Vec3::new(
    //                 current_camera_based_on_world_origin.x,
    //                 0.0,
    //                 current_camera_based_on_world_origin.y,
    //             ),
    //             Vec3::NEG_Z,
    //         ),
    //         camera: Camera {
    //             target: RenderTarget::Window(WindowRef::Entity(window_entity)),
    //             ..default()
    //         },
    //         ..default()
    //     });
    // }
}
