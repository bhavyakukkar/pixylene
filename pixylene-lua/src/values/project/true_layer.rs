use crate::{
    utils::{CanvasMismatch, ContextExpired, BOXED_ERROR, CANVAS_MISMATCH_TRUE, LAYER_GONE},
    values::{
        project::TrueScene,
        types::{BlendMode, PCoord},
    },
    Context,
};

use libpixylene::{project, types};
use std::{
    borrow::{Borrow, BorrowMut},
    sync::Arc,
};
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, Error::ExternalError, FromLua, Lua, MetaMethod, Result,
            UserData, UserDataFields, UserDataMethods,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};

/// Lua interface to libpixylene's [`Layer`][project::Layer] type
#[derive(Clone)]
pub struct TrueLayer(pub Context<project::Layer<types::TruePixel>, u16>);

impl<'lua> FromLua<'lua> for TrueLayer {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<TrueLayer> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<TrueLayer>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "TrueLayer",
                message: None,
            }),
        }
    }
}

impl TealData for TrueLayer {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A Scene with extra opacity, mute and blend-mode information.");

        //Lua interface to construct a new Layer
        {
            mlua_create_named_parameters!(
                TrueLayerArgs with
                    scene: TrueScene,
                    opacity: u8,
                    mute: bool,
                    blend_mode: BlendMode,
            );
            methods.document(
                "Create & return a new TrueLayer by providing a TrueScene (which will exist \
                             as a clone in the TrueLayer), an opacity (0-255), whether it is muted \
                             (boolean), and its BlendMode",
            );
            methods.add_meta_method(MetaMethod::Call, |_, _, a: TrueLayerArgs| {
                Ok(TrueLayer(Context::Solo(project::Layer {
                    scene: a.scene.0.do_imt::<_, _, CanvasMismatch<
                        ContextExpired<project::Scene<types::TruePixel>>,
                    >>(|scene| Ok(Ok(scene.clone())))(
                        |pixylene, index| {
                            pixylene.project.canvas.layers.to_true().map(|layers| {
                                layers
                                    .get_layer(*index)
                                    .map(|layer| layer.scene.clone())
                                    .map_err(|_| ())
                            })
                        },
                    )
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?,
                    opacity: a.opacity,
                    mute: a.mute,
                    blend_mode: a.blend_mode.0,
                })))
            });
        }

        //Lua interface to Layer::merge
        {
            mlua_create_named_parameters!(
                TrueLayerMergeArgs with
                    dimensions: PCoord,
                    top: TrueLayer,
                    bottom: TrueLayer,
                    blend_mode: BlendMode,
            );
            methods.document(
                "Create & return a new TrueLayer of given dimensions by merging the two \
                existing layers with the given blend_mode",
            );
            methods.add_function("merge", |_, a: TrueLayerMergeArgs| {
                let top = a
                    .top
                    .0
                    .do_imt::<_, _, CanvasMismatch<ContextExpired<project::Layer>>>(|layer| {
                        Ok(Ok(layer.clone()))
                    })(|pixylene, index| {
                    pixylene.project.canvas.layers.to_true().map(|layers| {
                        layers
                            .get_layer(*index)
                            .map(|layer| layer.clone())
                            .map_err(|_| ())
                    })
                })
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?;

                let bottom = a
                    .bottom
                    .0
                    .do_imt::<_, _, CanvasMismatch<ContextExpired<project::Layer>>>(|layer| {
                        Ok(Ok(layer.clone()))
                    })(|pixylene, index| {
                    pixylene.project.canvas.layers.to_true().map(|layers| {
                        layers
                            .get_layer(*index)
                            .map(|layer| layer.clone())
                            .map_err(|_| ())
                    })
                })
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?;

                project::Layer::merge(a.dimensions.0, &top, &bottom, a.blend_mode.0)
                    .map(|scene| TrueScene(Context::Solo(scene)))
                    .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
            });
        }

        // todo: + metamethod alias for merge that checks for consistent sizes and uses top layer's
        //       blend_mode

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        use Context::*;

        //Lua interface to Layer.scene
        fields.document("the Scene being composed by this TrueLayer");
        fields.add_field_method_get("scene", |_, this| match &this.0 {
            Solo(layer) => Ok(TrueScene(Solo(layer.scene.clone()))),
            Linked(pixylene, index) => Ok(TrueScene(Linked(pixylene.clone(), *index))),
        });
        fields.add_field_method_set("scene", |_, this, scene: TrueScene| {
            let that_scene = scene.0.do_imt::<_, _, CanvasMismatch<
                ContextExpired<project::Scene<types::TruePixel>>,
            >>(|scene| Ok(Ok(scene.clone())))(|pixylene, index| {
                pixylene
                    .borrow()
                    .project
                    .canvas
                    .layers
                    .to_true()
                    .map(|layers| {
                        layers
                            .get_layer(*index)
                            .map(|layer| layer.scene.clone())
                            .map_err(|_| ())
                    })
            })
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?;

            this.0
                .do_mut::<_, _, CanvasMismatch<ContextExpired<()>>>(|layer| {
                    layer.scene = that_scene.clone();
                    Ok(Ok(()))
                })(|mut pixylene, index| {
                pixylene
                    .borrow_mut()
                    .project
                    .canvas
                    .layers
                    .to_true_mut()
                    .map(|layers| {
                        layers
                            .get_layer_mut(*index)
                            .map(|layer| {
                                layer.scene = that_scene.clone();
                            })
                            .map_err(|_| ())
                    })
            })
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))
        });

        //Lua interface to Layer.opacity
        fields.document("the opacity of this TrueLayer");
        fields.add_field_method_get("opacity", |_, this| {
            this.0
                .do_imt::<_, _, CanvasMismatch<ContextExpired<u8>>>(|layer| Ok(Ok(layer.opacity)))(
                |pixylene, index| {
                    pixylene.project.canvas.layers.to_true().map(|layers| {
                        layers
                            .get_layer(*index)
                            .map(|layer| layer.opacity)
                            .map_err(|_| ())
                    })
                },
            )
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))
        });
        fields.add_field_method_set("opacity", |_, this, opacity| {
            this.0
                .do_mut::<_, _, CanvasMismatch<ContextExpired<()>>>(|layer| {
                    layer.opacity = opacity;
                    Ok(Ok(()))
                })(|mut pixylene, index| {
                pixylene.project.canvas.layers.to_true_mut().map(|layers| {
                    layers
                        .get_layer_mut(*index)
                        .map(|layer| {
                            layer.opacity = opacity;
                        })
                        .map_err(|_| ())
                })
            })
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))
        });

        //Lua interface to Layer.mute
        fields.document("the mute of this TrueLayer");
        fields.add_field_method_get("mute", |_, this| {
            this.0
                .do_imt::<_, _, CanvasMismatch<ContextExpired<bool>>>(|layer| Ok(Ok(layer.mute)))(
                |pixylene, index| {
                    pixylene.project.canvas.layers.to_true().map(|layers| {
                        layers
                            .get_layer(*index)
                            .map(|layer| layer.mute)
                            .map_err(|_| ())
                    })
                },
            )
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))
        });
        fields.add_field_method_set("mute", |_, this, mute| {
            this.0
                .do_mut::<_, _, CanvasMismatch<ContextExpired<()>>>(|layer| {
                    layer.mute = mute;
                    Ok(Ok(()))
                })(|mut pixylene, index| {
                pixylene.project.canvas.layers.to_true_mut().map(|layers| {
                    layers
                        .get_layer_mut(*index)
                        .map(|layer| {
                            layer.mute = mute;
                        })
                        .map_err(|_| ())
                })
            })
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))
        });

        //Lua interface to Layer.blend_mode
        fields.document("the blend-mode of this TrueLayer");
        fields.add_field_method_get("blend_mode", |_, this| {
            Ok(BlendMode(
                this.0
                    .do_imt::<_, _, CanvasMismatch<ContextExpired<types::BlendMode>>>(|layer| {
                        Ok(Ok(layer.blend_mode))
                    })(|pixylene, index| {
                    pixylene.project.canvas.layers.to_true().map(|layers| {
                        layers
                            .get_layer(*index)
                            .map(|layer| layer.blend_mode)
                            .map_err(|_| ())
                    })
                })
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?,
            ))
        });
        fields.add_field_method_set("blend_mode", |_, this, blend_mode: BlendMode| {
            this.0
                .do_mut::<_, _, CanvasMismatch<ContextExpired<()>>>(|layer| {
                    layer.blend_mode = blend_mode.0;
                    Ok(Ok(()))
                })(|mut pixylene, index| {
                pixylene.project.canvas.layers.to_true_mut().map(|layers| {
                    layers
                        .get_layer_mut(*index)
                        .map(|layer| {
                            layer.blend_mode = blend_mode.0;
                        })
                        .map_err(|_| ())
                })
            })
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))
        });
    }
}

impl ToTypename for TrueLayer {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("TrueLayer", tealr::KindOfType::External)
    }
}

impl UserData for TrueLayer {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for TrueLayer {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
