use crate::util::{single, single_mut};
use crate::{
    level_instantiation::on_spawn::{player, Player},
    player_control::camera::IngameCamera,
};
use bevy::prelude::*;

pub(super) fn set_camera_focus(
    mut camera_query: Query<&mut IngameCamera>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut camera = single_mut!(camera_query);
    let player_transform = single!(player_query);
    camera.target = player_transform.translation + Vec3::Y * player::HEIGHT / 2.;
}
