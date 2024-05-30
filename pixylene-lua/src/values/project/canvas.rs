use crate::{
    utils::{CanvasMismatch, CANVAS_MISMATCH_TRUE, CANVAS_MISMATCH_INDEXED, BOXED_ERROR},
    values::{
        project::{TrueScene, IndexedScene, TrueLayers, IndexedLayers, Palette},
        types::{PCoord, TruePixel, IndexedPixel},
    },
    Context,
};

use libpixylene::{types, project};
use std::sync::Arc;
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, FromLua, Lua, UserData, UserDataFields,
            UserDataMethods, Error::ExternalError
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};

/// Mixed Lua interface to libpixylene's [`Canvas`][project::Canvas] &
/// [`LayersType`][project::LayersType] types
#[derive(Clone)]
pub struct Canvas(pub Context<project::Canvas, ()>);

impl<'lua> FromLua<'lua> for Canvas {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> mlua::Result<Canvas> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<Canvas>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Canvas",
                message: None,
            }),
        }
    }
}

impl TealData for Canvas {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A set of true-color or indexed-color Layers with uniform dimensions
                              and a Palette.");

        //Lua interface to construct a Canvas containing TrueLayers
        {
            mlua_create_named_parameters!(
                CanvasTrueArgs with
                    layers: TrueLayers,
                    palette: Palette,
            );
            methods.document(
                "Construct and return a new true Canvas from TrueLayers and a Palette"
            );
            methods.add_function("true", |_, a: CanvasTrueArgs| {
                let layers = a.layers.0.do_imt::<_, _, CanvasMismatch<
                    project::Layers<types::TruePixel>
                >>
                    (|layers| Ok(layers.clone()))
                    (|pixylene, _| pixylene.project.canvas.layers.to_true()
                        .map(|layers| layers.clone()))
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?;

                let palette = a.palette.0.do_imt
                    (|palette| palette.clone())
                    (|pixylene, _| pixylene.project.canvas.palette.clone());

                Ok(Canvas(Context::Solo(project::Canvas{
                    layers: project::LayersType::True(layers),
                    palette
                })))
            });
        }

        //Lua interface to construct a Canvas containing IndexedLayers
        {
            mlua_create_named_parameters!(
                CanvasTrueArgs with
                    layers: IndexedLayers,
                    palette: Palette,
            );
            methods.document(
                "Construct and return a new indexed Canvas from IndexedLayers and a Palette"
            );
            methods.add_function("true", |_, a: CanvasTrueArgs| {
                let layers = a.layers.0.do_imt::<_, _, CanvasMismatch<
                    project::Layers<types::IndexedPixel>
                >>
                    (|layers| Ok(layers.clone()))
                    (|pixylene, _| pixylene.project.canvas.layers.to_indexed()
                        .map(|layers| layers.clone()))
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?;

                let palette = a.palette.0.do_imt
                    (|palette| palette.clone())
                    (|pixylene, _| pixylene.project.canvas.palette.clone());

                Ok(Canvas(Context::Solo(project::Canvas{
                    layers: project::LayersType::Indexed(layers),
                    palette
                })))
            });
        }

        //Lua interface to merged_true_scene()
        {
            mlua_create_named_parameters!(
                CanvasMergeArgs with
                    background: Option<TruePixel>,
            );
            methods.document(
                "Merges the layers of the Canvas into a TrueScene with an optional background \
                color",
            );
            methods.add_method("merge_true", |_, this, a: CanvasMergeArgs| {
                let color = a.background.map(|color| color.0);
                Ok(TrueScene(Context::Solo(this.0.do_imt
                    (|canvas| canvas.merged_true_scene(color))
                    (|pixylene, _| pixylene.project.canvas.merged_true_scene(color))
                )))
            });
        }


        //Lua interface to merged_indexed_scene()
        {
            mlua_create_named_parameters!(
                CanvasMergeArgs with
                    background: Option<IndexedPixel>,
            );
            methods.document(
                "Merges the layers of the Canvas into a IndexedScene with an optional background \
                color",
            );
            methods.add_method("merge_indexed", |_, this, a: CanvasMergeArgs| {
                let color = a.background.map(|color| color.0);
                Ok(IndexedScene(Context::Solo(this.0.do_imt
                    (|canvas| canvas.merged_indexed_scene(color))
                    (|pixylene, _| pixylene.project.canvas.merged_indexed_scene(color))
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
                )))
            });
        }
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        //Lua interface to Canvas field palette
        fields.document("the palette composed by this Canvas");
        fields.add_field_method_get("palette", |_, this| Ok(match &this.0 {
            Context::Solo(ref canvas) => Palette(Context::Solo(canvas.palette.clone())),
            Context::Linked(pixylene, _) => Palette(Context::Linked(pixylene.clone(), ())),
        }));
        fields.add_field_method_set("palette", |_, this, palette: Palette| {
            let new_palette = palette.0.do_imt
                (|palette| palette.clone())
                (|pixylene, _| pixylene.project.canvas.palette.clone());

            this.0.do_mut
                (|canvas| {
                    canvas.palette = new_palette.clone();
                })
                (|mut pixylene, _| {
                    pixylene.project.canvas.palette = new_palette.clone();
                });
            Ok(())
        });

        //Lua interface to find out whether Canvas contains indexed layers or not
        fields.document("whether the Canvas is indexed");
        fields.add_field_method_get("indexed", |_, this| Ok(this.0.do_imt
            (|canvas| matches!(canvas.layers, project::LayersType::Indexed(_)))
            (|pixylene, _|
                matches!(pixylene.project.canvas.layers, project::LayersType::Indexed(_))))
        );

        fields.document("the layers of the Canvas, returned in a table that either contains the \
                        composed TrueLayers to key \"true\" or the composed IndexedLayers to key \
                        \"indexed\"");
        fields.add_field_method_get("layers", |lua_ctx, this| {
            let table = lua_ctx.create_table()?;
            match &this.0 {
                Context::Solo(ref canvas) => match &canvas.layers {
                    project::LayersType::True(layers) => {
                        table.set("true", TrueLayers(Context::Solo(layers.clone())))?;
                    },
                    project::LayersType::Indexed(layers) => {
                        table.set("indexed", IndexedLayers(Context::Solo(layers.clone())))?;
                    },
                },
                Context::Linked(pixylene, _) => match &pixylene.borrow().project.canvas.layers {
                    project::LayersType::True(_) => {
                        table.set("true", TrueLayers(Context::Linked(pixylene.clone(), ())))?;
                    },
                    project::LayersType::Indexed(_) => {
                        table.set("indexed", IndexedLayers(Context::Linked(pixylene.clone(), ())))?;
                    },
                }
            }
            Ok(table)
        });

        //Lua interface to LayersType method dim
        fields.document("the dimensions of the layers in the Canvas");
        fields.add_field_method_get("dim", |_, this| Ok(PCoord(this.0.do_imt
            (|canvas| canvas.layers.dim())
            (|pixylene, _| pixylene.project.canvas.layers.dim())
        )));

        //Lua interface to LayersType method len
        fields.document("the number of layers in the Canvas");
        fields.add_field_method_get("len", |_, this| Ok(this.0.do_imt
            (|canvas| canvas.layers.len())
            (|pixylene, _| pixylene.project.canvas.layers.len())
        ));
    }
}

impl ToTypename for Canvas {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Canvas", tealr::KindOfType::External)
    }
}

impl UserData for Canvas {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Canvas {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
