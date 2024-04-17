use crate::{
    Context,
    utils::layer_gone,
    values::{
        project::Scene,
        types::{BlendMode, PCoord},
    }
};

use libpixylene::project;
use std::sync::Arc;
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, FromLua, Lua, MetaMethod, Result, UserData, UserDataFields,
            UserDataMethods,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};

/// Lua interface to libpixylene's [`Layer`][project::Layer] type
#[derive(Clone)]
pub struct Layer(pub Context<project::Layer, u16>);

impl<'lua> FromLua<'lua> for Layer {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Layer> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<Layer>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Layer",
                message: None,
            }),
        }
    }
}

impl TealData for Layer {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A Scene with extra opacity, mute and blend-mode information.");

        //Lua interface to construct a new Layer
        {
            mlua_create_named_parameters!(
                LayerArgs with
                    scene: Scene,
                    opacity: u8,
                    mute: bool,
                    blend_mode: BlendMode,
            );
            methods.document(
                "Create & return a new Layer by providing a Scene (which will exist \
                             as a clone in the Layer), an opacity (0-255), whether it is muted \
                             (boolean), and its BlendMode",
            );
            methods.add_meta_method(MetaMethod::Call, |_, _, a: LayerArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                Ok(Layer(Context::Solo(project::Layer {
                    scene: a.scene.0.do_imt(
                        |scene| Ok(scene.clone()),
                        |pixylene, index| pixylene.project.canvas
                            .get_layer(*index)
                            .map(|layer| layer.scene.clone())
                    ).map_err(|_| ExternalError(Arc::from(boxed_error(layer_gone))))?,
                    opacity: a.opacity,
                    mute: a.mute,
                    blend_mode: a.blend_mode.0,
                })))
            });
        }

        //Lua interface to Layer::merge
        {
            mlua_create_named_parameters!(
                LayerMergeArgs with
                    dimensions: PCoord,
                    top: Layer,
                    bottom: Layer,
                    blend_mode: BlendMode,
            );
            methods.document(
                "Create & return a new Layer of given dimensions by merging the two \
                             existing layers with the given blend_mode",
            );
            methods.add_function("merge", |_, a: LayerMergeArgs| {
                use mlua::Error::ExternalError;
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                let top = a.top.0.do_imt(
                    |layer| Ok(layer.clone()),
                    |pixylene, index| pixylene.project.canvas.get_layer(*index).map(|layer| layer.clone()),
                ).map_err(|_| ExternalError(Arc::from(boxed_error(layer_gone))))?;
                
                let bottom = a.bottom.0.do_imt(
                    |layer| Ok(layer.clone()),
                    |pixylene, index| pixylene.project.canvas.get_layer(*index).map(|layer| layer.clone()),
                ).map_err(|_| ExternalError(Arc::from(boxed_error(layer_gone))))?;

                match project::Layer::merge(a.dimensions.0, &top, &bottom, a.blend_mode.0) {
                    Ok(scene) => Ok(Scene(Context::Solo(scene))),
                    Err(err) => Err(ExternalError(Arc::from(boxed_error(&err.to_string())))),
                }
            });
        }

        // todo: + metamethod alias for merge that checks for consistent sizes and uses top layer's
        //       blend_mode

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        use Context::*;
        use mlua::Error::{ ExternalError };

        //Lua interface to Layer.scene
        fields.document("the Scene being composed by this Layer");
        fields.add_field_method_get("scene", |_, this| match &this.0 {
            Solo(layer) => Ok(Scene(Solo(layer.scene.clone()))),
            Linked(pixylene, index) => Ok(Scene(Linked(pixylene.clone(), *index))),
        });
        fields.add_field_method_set("scene", |_, this, scene: Scene| {
            let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

            match &mut this.0 {
                Solo(ref mut layer) => match scene.0 {
                    Solo(scene) => {
                        layer.scene = scene.clone();
                        Ok(())
                    },
                    Linked(pixylene, index) => {
                        match pixylene.borrow().project.canvas.get_layer(index) {
                            Ok(layer2) => {
                                layer.scene = layer2.scene.clone();
                                Ok(())
                            },
                            Err(_) => Err(ExternalError(Arc::from(boxed_error(layer_gone)))),
                        }
                    },
                },
                Linked(pixylene, index) => scene.0.do_imt(
                    |scene| {
                        match pixylene.borrow_mut().project.canvas.get_layer_mut(*index) {
                            Ok(layer) => {
                                layer.scene = scene.clone();
                                Ok(())
                            },
                            Err(_) => Err(ExternalError(Arc::from(boxed_error(layer_gone)))),
                        }
                    },
                    |pixylene2, index2| {
                        match pixylene.borrow_mut().project.canvas.get_layer_mut(*index) {
                            Ok(layer) => {
                                match pixylene2.project.canvas.get_layer(*index2) {
                                    Ok(layer2) => {
                                        layer.scene = layer2.scene.clone();
                                        Ok(())
                                    },
                                    Err(_) => Err(ExternalError(Arc::from(boxed_error(layer_gone)))),
                                }
                            },
                            Err(_) => Err(ExternalError(Arc::from(boxed_error(layer_gone)))),
                        }
                    },
                )
            }
        });

        //Lua interface to Layer.opacity
        fields.document("the opacity of this Layer");
        fields.add_field_method_get("opacity", |_, this| this.0.do_imt(
            |layer| Ok(layer.opacity),
            |pixylene, index| pixylene.project.canvas.get_layer(*index).map(|layer| layer.opacity)
        ).map_err(|_|
            ExternalError(Arc::from(Box::<dyn std::error::Error + Send + Sync>::from(layer_gone)))
        ));
        fields.add_field_method_set("opacity", |_, this, opacity| this.0.do_mut(
            |layer| {
                layer.opacity = opacity;
                Ok(())
            },
            |mut pixylene, index| pixylene.project.canvas.get_layer_mut(*index).map(|layer| {
                layer.opacity = opacity;
            })
        ).map_err(|_|
            ExternalError(Arc::from(Box::<dyn std::error::Error + Send + Sync>::from(layer_gone)))
        ));

        //Lua interface to Layer.mute
        fields.document("whether this Layer is muted");
        fields.add_field_method_get("mute", |_, this| this.0.do_imt(
            |layer| Ok(layer.mute),
            |pixylene, index| pixylene.project.canvas.get_layer(*index).map(|layer| layer.mute)
        ).map_err(|_|
            ExternalError(Arc::from(Box::<dyn std::error::Error + Send + Sync>::from(layer_gone)))
        ));
        fields.add_field_method_set("mute", |_, this, mute| this.0.do_mut(
            |layer| {
                layer.mute = mute;
                Ok(())
            },
            |mut pixylene, index| pixylene.project.canvas.get_layer_mut(*index).map(|layer| {
                layer.mute = mute;
            })
        ).map_err(|_|
            ExternalError(Arc::from(Box::<dyn std::error::Error + Send + Sync>::from(layer_gone)))
        ));

        //Lua interface to Layer.blend_mode
        fields.document("the blend-mode of this Layer");
        fields.add_field_method_get("blend_mode", |_, this| this.0.do_imt(
            |layer| Ok(BlendMode(layer.blend_mode)),
            |pixylene, index| pixylene.project.canvas.get_layer(*index)
            .map(|layer| BlendMode(layer.blend_mode))
        ).map_err(|_|
            ExternalError(Arc::from(Box::<dyn std::error::Error + Send + Sync>::from(layer_gone)))
        ));
        fields.add_field_method_set("blend_mode", |_, this, blend_mode: BlendMode| this.0.do_mut(
            |layer| {
                layer.blend_mode = blend_mode.0;
                Ok(())
            },
            |mut pixylene, index| pixylene.project.canvas.get_layer_mut(*index).map(|layer| {
                layer.blend_mode = blend_mode.0;
            })
        ).map_err(|_|
            ExternalError(Arc::from(Box::<dyn std::error::Error + Send + Sync>::from(layer_gone)))
        ));
    }
}

impl ToTypename for Layer {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Layer", tealr::KindOfType::External)
    }
}

impl UserData for Layer {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Layer {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
