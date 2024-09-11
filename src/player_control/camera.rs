use crate::SystemSet;
use crate::{player_control::camera::focus::set_camera_focus, AppState};
use bevy::prelude::*;

#[cfg(not(feature = "web"))]
use bevy_atmosphere::prelude::AtmospherePlugin;

use serde::{Deserialize, Serialize};
use ui::*;

mod focus;
mod ui;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct IngameCamera {
    pub(crate) target: Vec3,
    pub(crate) secondary_target: Option<Vec3>,
    pub(crate) desired_distance: f32,
    pub(crate) kind: IngameCameraKind,
}

impl Default for IngameCamera {
    fn default() -> Self {
        Self {
            desired_distance: 5.,
            target: default(),
            secondary_target: default(),
            kind: default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) enum IngameCameraKind {
    #[default]
    ThirdPerson,
    FirstPerson,
    FixedAngle,
}

/// Handles the main ingame camera, i.e. not the UI camera in the menu.
/// Cameras are controlled with [`CameraAction`](crate::player_control::actions::CameraAction). Depending on the distance, a first person,
/// third person or fixed angle camera is used.
pub(super) fn plugin(app: &mut App) {
    #[cfg(not(feature = "web"))]
    app.add_plugins(AtmospherePlugin);

    app.register_type::<UiCamera>()
        .register_type::<IngameCamera>()
        .register_type::<IngameCameraKind>()
        .add_systems(Startup, spawn_ui_camera)
        .add_systems(OnEnter(AppState::Playing), despawn_ui_camera);

    //https://github.com/dimforge/bevy_rapier/issues/564
    app.add_systems(
        PostUpdate,
        (
            // grab_cursor,
            set_camera_focus,
            // Dolly::<IngameCamera>::update_active,
        )
            .chain()
            .in_set(SystemSet::CameraUpdate),
    );
}
