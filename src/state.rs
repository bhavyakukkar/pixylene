pub enum Root {
    NoSceneLoaded,
    SceneLoaded(SceneLoaded)
}
pub enum SceneLoaded {
    LayerView,
    CameraView,
    PaletteView
}

/*
fn get_keybind_tree(mode: Main) {
    match mode {
        Root::NoSceneLoaded => todo!(),
        Root::SceneLoaded(mode) => {
            match mode {
                SceneLoaded::LayerView => todo!(),
                SceneLoaded::CameraView => todo!(),
                SceneLoaded::PaletteView => todo!()
            }
        }
    }
}
*/

/*
 * KeyTree: map<key, action> = KeyTrees[Mode];
 */
