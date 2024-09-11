use bevy::prelude::*;

use bevy_panorbit_camera::PanOrbitCameraPlugin;

pub(super) fn plugin(app: &mut App) {
    {
        app.add_plugins(PanOrbitCameraPlugin);
    }
}
