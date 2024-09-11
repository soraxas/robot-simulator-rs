use crate::AppState;
use bevy::prelude::*;
use bevy_gltf_blueprints::GltfBlueprintsSet;

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            GltfBlueprintsSet::AfterSpawn,
            SystemSet::ColliderSpawn,
            SystemSet::Navigation,
            SystemSet::PlayerEmbodiment,
            SystemSet::GeneralMovement,
            SystemSet::PlayAnimation,
            SystemSet::UpdateInteractionOpportunities,
            SystemSet::Dialog,
        )
            .chain(),
    )
    .configure_sets(
        Update,
        (
            SystemSet::ColliderSpawn,
            SystemSet::UpdateInteractionOpportunities,
            SystemSet::Navigation,
            SystemSet::PlayerEmbodiment,
            SystemSet::GeneralMovement,
            SystemSet::PlayAnimation,
            SystemSet::Dialog,
        )
            .run_if(in_state(AppState::Playing)),
    );

    //https://github.com/dimforge/bevy_rapier/issues/564

    app.configure_sets(
        PostUpdate,
        (SystemSet::CameraUpdate)
            .chain()
            .after(bevy_rapier3d::plugin::PhysicsSet::Writeback)
            .before(bevy::transform::TransformSystem::TransformPropagate)
            .run_if(in_state(AppState::Playing)),
    );
}

/// Used for ordering systems across Foxtrot.
/// Note that the order of items of this enum is not necessarily the order of execution.
/// Look at [`crate::system_set::plugin`] for the actual order used.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) enum SystemSet {
    /// Goes through entities tagged with `Collider` in Blender
    /// and inserts a proper XPBD collider
    ColliderSpawn,
    /// Run path finding
    Navigation,
    /// Update interaction opportunities with the environment
    UpdateInteractionOpportunities,
    /// Handle player input
    PlayerEmbodiment,
    /// Handle movement for character controllers
    GeneralMovement,
    /// Play animations
    PlayAnimation,
    /// Update the camera transform
    CameraUpdate,
    /// Interacts with Yarn Spinner for dialog logic
    Dialog,
}
