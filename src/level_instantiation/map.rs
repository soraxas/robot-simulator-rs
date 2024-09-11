use crate::{file_system_interaction::asset_loading::GltfAssets, AppState};
use bevy::{gltf::Gltf, prelude::*};

use bevy_panorbit_camera::PanOrbitCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Playing), spawn_level);
}

fn spawn_level(mut commands: Commands, models: Res<Assets<Gltf>>, gltf_assets: Res<GltfAssets>) {
    let gltf = models.get(&gltf_assets.level).unwrap();
    commands.spawn((
        SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..default()
        },
        Name::new("Level"),
    ));

    // commands.spawn((
    //     Name::new("Camera"),
    //     Camera3dBundle::default(),
    //     IngameCamera::default(),
    //     AtmosphereCamera::default(),
    //     IsDefaultUiCamera,
    //     create_camera_action_input_manager_bundle(),
    // ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
