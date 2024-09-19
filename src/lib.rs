use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_egui::EguiPlugin;

pub mod assets_loader;
pub mod camera;
pub mod collision_checker;
pub mod dev;
pub mod robot;
pub mod robot_vis;
pub mod scene;
pub mod util;

pub mod engine;

pub struct SimPlugin;



impl PluginGroup for SimPlugin {

    fn build(self) -> PluginGroupBuilder{

        let mut group = PluginGroupBuilder::start::<Self>();


        group = group.add_group(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "RobotSim".to_string(),
                    // title: "Bevy Rust Experiments".to_string(),
                    resizable: true,
                    // cursor_visible: true,
                    // present_mode: PresentMode::AutoVsync,
                    // This will spawn an invisible window
                    fit_canvas_to_parent: true, // no more need to handle this myself with wasm binding: https://github.com/bevyengine/bevy/commit/fed93a0edce9d66586dc70c1207a2092694b9a7d
                    canvas: Some("#bevy".to_string()),

                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    // visible: false,
                    ..default()
                }
                .into(),
                ..default()
            }),
        )
        // .add_plugins(web_demo::plugin)
        .add(dev::plugin);

        // if !app.is_plugin_added::<EguiPlugin>() {
        //     app.add_plugins(EguiPlugin);
        // }

        group = group
            // .add_plugins(EguiPlugin)
            .add(camera::plugin) // camera needs egui to be added first
            .add(scene::plugin)
            .add(robot_vis::plugin);

        group
    }
}
