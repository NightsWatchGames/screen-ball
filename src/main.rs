use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, Cursor, WindowLevel, WindowResolution};
use avian3d::prelude::*;

mod area;
mod ball;
mod camera;
mod util;

fn main() {
    // 计算屏幕大小和窗口位置
    let primary_display = util::primary_display();

    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                transparent: true,
                decorations: false,
                window_level: WindowLevel::AlwaysOnTop,
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                // windows系统上窗口不能跟显示器一样大，会导致背景不是透明（为黑色）
                resolution: WindowResolution::new(
                    primary_display.width as f32 * 0.99,
                    primary_display.height as f32 * 0.99,
                ),
                position: WindowPosition::At(IVec2::new(0, 0)),
                cursor: Cursor {
                    hit_test: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_systems(
            Startup,
            (camera::setup_camera, area::setup_area, ball::setup_ball),
        )
        .add_systems(Update, (ball::play_ball, area::update_wall))
        .run();
}
