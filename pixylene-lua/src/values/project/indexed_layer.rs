use crate::{
    Context,
    utils::{CanvasMismatch, ContextExpired, CANVAS_MISMATCH_INDEXED, LAYER_GONE, BOXED_ERROR},
    values::{
        project::IndexedScene,
        types::BlendMode,
    }
};

use libpixylene::{types, project};
use std::{borrow::{Borrow, BorrowMut}, sync::Arc};
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, FromLua, Lua, MetaMethod, Result, UserData, UserDataFields,
            UserDataMethods, Error::ExternalError
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};


/// Lua interface to libpixylene's [`Layer`][project::Layer] type
#[derive(Clone)]
pub struct IndexedLayer(pub Context<project::Layer<types::IndexedPixel>, u16>);

impl<'lua> FromLua<'lua> for IndexedLayer {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<IndexedLayer> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<IndexedLayer>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "IndexedLayer",
                message: None,
            }),
        }
    }
}

impl TealData for IndexedLayer {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A Scene with extra opacity, mute and blend-mode information.");

        //Lua interface to construct a new Layer
        {
            mlua_create_named_parameters!(
                IndexedLayerArgs with
                    scene: IndexedScene,
                    opacity: u8,
                    mute: bool,
                    blend_mode: BlendMode,
            );
            methods.document(
                "Create & return a new IndexedLayer by providing a IndexedScene (which will exist \
                             as a clone in the IndexedLayer), an opacity (0-255), whether it is muted \
                             (boolean), and its BlendMode",
            );
            methods.add_meta_method(MetaMethod::Call, |_, _, a: IndexedLayerArgs| {
                Ok(IndexedLayer(Context::Solo(project::Layer {
                    scene: a.scene.0.do_imt::<_, _, CanvasMismatch<ContextExpired<
                        project::Scene<types::IndexedPixel>
                    >>>
                        (|scene| Ok(Ok(scene.clone())))
                        (|pixylene, index| pixylene.project.canvas.layers.to_indexed()
                            .map(|layers| layers.get_layer(*index)
                                .map(|layer| layer.scene.clone())
                                .map_err(|_| ())))
                        .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
                        .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?,
                    opacity: a.opacity,
                    mute: a.mute,
                    blend_mode: a.blend_mode.0,
                })))
            });

            //todo: Lua interface to Layer<IndexedPixel>::to_true_layer
            {
            }
        }

        // todo: + metamethod alias for merge that checks for consistent sizes and uses top layer's
        //       blend_mode

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        use Context::*;

        //Lua interface to Layer.scene
        fields.document("the Scene being composed by this IndexedLayer");
        fields.add_field_method_get("scene", |_, this| match &this.0 {
            Solo(layer) => Ok(IndexedScene(Solo(layer.scene.clone()))),
            Linked(pixylene, index) => Ok(IndexedScene(Linked(pixylene.clone(), *index))),
        });
        fields.add_field_method_set("scene", |_, this, scene: IndexedScene| {
            let that_scene = scene.0.do_imt::<_, _, CanvasMismatch<ContextExpired<
                project::Scene<types::IndexedPixel>
            >>>
                (|scene| Ok(Ok(scene.clone())))
                (|pixylene, index| pixylene.borrow().project.canvas.layers.to_indexed()
                    .map(|layers| layers.get_layer(*index)
                         .map(|layer| layer.scene.clone())
                         .map_err(|_| ())))
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?;

            this.0.do_mut::<_, _, CanvasMismatch<ContextExpired<()>>>
                (|layer| {
                    layer.scene = that_scene.clone();
                    Ok(Ok(()))
                })
                (|mut pixylene, index| pixylene.borrow_mut().project.canvas.layers.to_indexed_mut()
                    .map(|layers| layers.get_layer_mut(*index)
                         .map(|layer| {
                             layer.scene = that_scene.clone();
                         })
                         .map_err(|_| ())))
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))
        });

        //Lua interface to Layer.opacity
        fields.document("the opacity of this IndexedLayer");
        fields.add_field_method_get("opacity", |_, this| this.0.do_imt::<_, _, CanvasMismatch<
            ContextExpired<u8>
        >>
            (|layer| Ok(Ok(layer.opacity)))
            (|pixylene, index| pixylene.project.canvas.layers.to_indexed()
                .map(|layers| layers.get_layer(*index)
                     .map(|layer| layer.opacity)
                     .map_err(|_| ())))
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE)))));
        fields.add_field_method_set("opacity", |_, this, opacity| this.0.do_mut::<_, _,
            CanvasMismatch<ContextExpired<()>>
        >
            (|layer| {
                layer.opacity = opacity;
                Ok(Ok(()))
            })
            (|mut pixylene, index| pixylene.project.canvas.layers.to_indexed_mut()
                .map(|layers| layers.get_layer_mut(*index)
                     .map(|layer| {
                         layer.opacity = opacity;
                     })
                     .map_err(|_| ())))
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE)))));

        //Lua interface to Layer.mute
        fields.document("the mute of this IndexedLayer");
        fields.add_field_method_get("mute", |_, this| this.0.do_imt::<_, _, CanvasMismatch<
            ContextExpired<bool>
        >>
            (|layer| Ok(Ok(layer.mute)))
            (|pixylene, index| pixylene.project.canvas.layers.to_indexed()
                .map(|layers| layers.get_layer(*index)
                     .map(|layer| layer.mute)
                     .map_err(|_| ())))
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE)))));
        fields.add_field_method_set("mute", |_, this, mute| this.0.do_mut::<_, _,
            CanvasMismatch<ContextExpired<()>>
        >
            (|layer| {
                layer.mute = mute;
                Ok(Ok(()))
            })
            (|mut pixylene, index| pixylene.project.canvas.layers.to_indexed_mut()
                .map(|layers| layers.get_layer_mut(*index)
                     .map(|layer| {
                         layer.mute = mute;
                     })
                     .map_err(|_| ())))
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE)))));

        //Lua interface to Layer.blend_mode
        fields.document("the blend-mode of this IndexedLayer");
        fields.add_field_method_get("blend_mode", |_, this| Ok(BlendMode(
            this.0.do_imt::<_, _, CanvasMismatch<
                ContextExpired<types::BlendMode>
            >>
                (|layer| Ok(Ok(layer.blend_mode)))
                (|pixylene, index| pixylene.project.canvas.layers.to_indexed()
                    .map(|layers| layers.get_layer(*index)
                         .map(|layer| layer.blend_mode)
                         .map_err(|_| ())))
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?)));
        fields.add_field_method_set("blend_mode", |_, this, blend_mode: BlendMode| this.0.do_mut::<_, _,
            CanvasMismatch<ContextExpired<()>>
        >
            (|layer| {
                layer.blend_mode = blend_mode.0;
                Ok(Ok(()))
            })
            (|mut pixylene, index| pixylene.project.canvas.layers.to_indexed_mut()
                .map(|layers| layers.get_layer_mut(*index)
                     .map(|layer| {
                         layer.blend_mode = blend_mode.0;
                     })
                     .map_err(|_| ())))
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE)))));
    }
}

impl ToTypename for IndexedLayer {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("IndexedLayer", tealr::KindOfType::External)
    }
}

impl UserData for IndexedLayer {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for IndexedLayer {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
