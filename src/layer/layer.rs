use crate::layer::{ Scene, Camera };
use crate::session::{ Session, SessionCamera };

pub struct Layer {
    pub scene: Scene,
    pub camera: Camera
}

impl Layer {
    pub fn new(session: &Session) -> Self {
        let mut scene = Scene::new(session.scene.dim, vec![session.scene.background; session.scene.dim.area() as usize], session.scene.background).unwrap();
        let camera = Camera::new(&scene, session.camera.dim, session.camera.focus, session.camera.mult, session.camera.repeat).unwrap();
        Self { scene: scene, camera: camera }
    }
    pub fn new_from_scene(session: &SessionCamera, scene: Scene) -> Self {
        let camera = Camera::new(&scene, session.dim, session.focus, session.mult, session.repeat).unwrap();
        Self { scene: scene, camera: camera }
    }
}
