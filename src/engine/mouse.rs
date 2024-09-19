use rapier3d::math::Point;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Copy, Clone, Debug, Resource)]
pub struct SceneMouse {
    pub ray: Option<(Point<f32>, rapier3d::math::Vector<f32>)>,
}

pub fn track_mouse_state(
    mut scene_mouse: ResMut<SceneMouse>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
) {
    if let Ok(window) = windows.get_single() {
        for (camera_transform, camera) in camera.iter() {
            if let Some(cursor) = window.cursor_position() {
                let ndc_cursor = ((cursor / Vec2::new(window.width(), window.height()) * 2.0)
                    - Vec2::ONE)
                    * Vec2::new(1.0, -1.0);
                let ndc_to_world =
                    camera_transform.compute_matrix() * camera.clip_from_view().inverse();
                let ray_pt1 =
                    ndc_to_world.project_point3(Vec3::new(ndc_cursor.x, ndc_cursor.y, -1.0));

                {
                    let ray_pt2 =
                        ndc_to_world.project_point3(Vec3::new(ndc_cursor.x, ndc_cursor.y, 1.0));
                    let ray_dir = ray_pt2 - ray_pt1;
                    scene_mouse.ray = Some((na::Vector3::from(ray_pt1).into(), ray_dir.into()));
                }
            }
        }
    }
}
