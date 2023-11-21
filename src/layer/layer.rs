use crate::layer::{ Scene, Camera };
use crate::session::{ SessionScene, SessionCamera };

pub struct Layer {
    pub scene: Scene,
    pub camera: Camera
}

impl Layer {
    pub fn new(session_scene: &SessionScene, session_camera: &SessionCamera) -> Result<Self, String> {
        let mut scene = Scene::new(session_scene.dim, vec![session_scene.background; session_scene.dim.area() as usize])?;
        let camera = Camera::new(&scene, session_camera.dim, session_camera.focus, session_camera.mult, session_camera.repeat)?;
        Ok(Self { scene: scene, camera: camera })
    }
    pub fn new_from_scene(session_camera: &SessionCamera, scene: Scene) -> Result<Self, String> {
        let camera = Camera::new(&scene, session_camera.dim, session_camera.focus, session_camera.mult, session_camera.repeat)?;
        Ok(Self { scene: scene, camera: camera })
    }
}
