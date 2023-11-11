use crate::scene::Scene;
use crate::camera::Camera;
use crate::color_picker::ColorPicker;

enum Action {
    Scene(fn(&mut Scene, &Camera, &ColorPicker)),
    Camera(fn(&mut Camera)),
    ColorPicker(fn(&mut ColorPicker)),
}

impl Action {
    //Actions that target Camera borrow Camera
    fn move_camera_up() -> Action {
        fn do_move_camera_up(camera: &mut Camera) {
            camera.focus.x -= 1;
        }
        Action::Camera(do_move_camera_up)
    }

    fn move_camera_left() -> Action {
        fn do_move_camera_left(camera: &mut Camera) {
            camera.focus.y -= 1;
        }
        Action::Camera(do_move_camera_left)
    }

    fn move_camera_down() -> Action {
        fn do_move_camera_down(camera: &mut Camera) {
            camera.focus.x += 1;
        }
        Action::Camera(do_move_camera_down)
    }

    fn move_camera_right() -> Action {
        fn do_move_camera_right(camera: &mut Camera) {
            camera.focus.y += 1;
        }
        Action::Camera(do_move_camera_right)
    }

    fn zoom_camera_in() -> Action {
        fn do_zoom_camera_in(camera: &mut Camera) {
            camera.mult += 1;
        }
        Action::Camera(do_zoom_camera_in)
    }

    fn zoom_camera_out() -> Action {
        fn do_zoom_camera_out(camera: &mut Camera) {
            camera.mult -= 1;
        }
        Action::Camera(do_zoom_camera_out)
    }

    //Actions that target Scene reference Camera, ColorPicker and borrow Scene
    fn draw_pixel() -> Action {
        fn do_draw_pixel(scene: &mut Scene, camera: &Camera, color_picker: &ColorPicker) {
            scene.set_pixel(camera.focus, color_picker.current);
        }
        Action::Scene(do_draw_pixel)
    }
    fn erase_pixel() -> Action {
        fn do_erase_pixel(scene: &mut Scene, camera: &Camera, color_picker: &ColorPicker) {
            scene.set_pixel(camera.focus, color_picker.empty);
        }
        Action::Scene(do_erase_pixel)
    }
}

/*fn main() {
    let action: Action = Action::zoom_camera_out();
}*/
