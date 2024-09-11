// disable console on windows for release builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use robotsim::SimPlugin;

fn main() {
    color_eyre::install().unwrap();

    App::new().add_plugins(SimPlugin).run();
}
