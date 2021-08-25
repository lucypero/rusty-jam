use bevy::prelude::*;

use crate::MainCamera;

pub fn mouse_system(
    windows: Res<Windows>,
    mut mouse_state: ResMut<MouseState>,
    q_camera: Query<&Transform, With<MainCamera>>
) {
    let window = windows.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = window.cursor_position() {
        // get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform
        let pos_world = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        mouse_state.world_position = Some(Vec2::new(pos_world.x, pos_world.y));
    } else {
        mouse_state.world_position = None;
    }
}

#[derive(Default)]
pub struct MouseState {
    pub world_position: Option<Vec2>,
}

impl MouseState {
    pub fn angle_from_location_to_mouse(&self, location: Vec2) -> Option<f32> {
        if let Some(mouse_position) = self.world_position {
            let mouse_offset = mouse_position - location;
            Some(Vec2::new(1.0, 0.0).angle_between(mouse_offset))
        }
        else {
            None
        }
    }
}
