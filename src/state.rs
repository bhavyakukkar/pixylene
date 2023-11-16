pub enum Main {
    NoSceneLoaded,
    SceneLoaded(SceneLoaded)
}

pub enum SceneLoaded {
    LayerView,
    CameraView,
    PaletteView
}

fn get_keybind_tree(mode: Main) {
    match mode {
        Main::NoSceneLoaded => todo!(),
        Main::SceneLoaded(mode) => {
            match mode {
                SceneLoaded::LayerView => todo!(),
                SceneLoaded::CameraView => todo!(),
                SceneLoaded::PaletteView => todo!()
            }
        }
    }
}

/*
 * KeyTree: map<key, action> = KeyTrees[Mode];
 */
