use std::io::Cursor;
use winit::window::Icon;

// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
// use bevy_inspector_egui::WorldInspectorPlugin;

use crate::GameState;

pub const ASPECT_RATIO: f32 = 5. / 8.;
pub const WIDTH: f32 = 90.;
pub const HEIGHT: f32 = WIDTH / ASPECT_RATIO;

pub struct ConfigPlugin;

#[derive(Component)]
pub struct CameraFlag;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 1 })
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        window: WindowDescriptor {
                            title: "beverage".to_string(),
                            canvas: Some("#bevy".to_owned()),
                            fit_canvas_to_parent: true,
                            width: 500.,
                            height: 800.,
                            ..default()
                        },
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
            )
            .add_startup_system(camera_setup)
            .add_startup_system(window_icon_setup);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(OverlayPlugin::default())
                // .add_plugin(FrameTimeDiagnosticsPlugin::default())
                // .add_plugin(LogDiagnosticsPlugin::default())
                // .add_plugin(WorldInspectorPlugin::new())
                .add_system(debug_system);
        }
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(WIDTH),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CameraFlag);
}

fn window_icon_setup(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/app_icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

fn debug_system(time: Res<Time>, windows: Res<Windows>, app_state: Res<State<GameState>>) {
    let current_time = time.elapsed_seconds();
    let at_interval = |t: f32| current_time % t < time.delta_seconds();
    if at_interval(1.) {
        let window = windows.get_primary().unwrap();
        screen_print!(col: Color::RED, "game state: {:?}", app_state.current());
        if let Some(position) = window.cursor_position() {
            screen_print!(col: Color::CYAN, "cursor_position: {}", position);
        };
    }
}

pub fn get_world_position(
    raw_position: Vec2,
    window: &Window,
    camera_transform: &GlobalTransform,
) -> Vec3 {
    let adjusted_position = Vec3::new(
        raw_position.x / window.width() * WIDTH - WIDTH / 2.,
        raw_position.y / window.height() * HEIGHT - HEIGHT / 2.,
        0.,
    );

    *camera_transform * adjusted_position
}
