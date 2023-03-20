use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, WindowLevel, WindowResolution};
use bevy_rapier3d::prelude::*;

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
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(camera::setup_camera)
        .add_startup_system(area::setup_area)
        .add_startup_system(ball::setup_ball)
        .add_system(ball::play_ball)
        .add_system(toggle_mouse_passthrough)
        .run();
}

// TODO 窗口不接收鼠标事件，将鼠标事件透给下面窗口
// 待 https://github.com/bevyengine/bevy/pull/7966 和 https://github.com/bevyengine/bevy/pull/7968 合入后移除此system
fn toggle_mouse_passthrough(keyboard_input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    if keyboard_input.just_pressed(KeyCode::P) {
        for mut window in &mut windows {
            window.cursor.hit_test = !window.cursor.hit_test;
        }
    }
}
