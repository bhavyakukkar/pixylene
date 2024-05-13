use crate::{
    utils::LAYER_GONE,
    values::{
        project::{Layer, Palette, Scene},
        types::{PCoord, TruePixel},
    },
    Context,
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

/// Lua interface to libpixylene's [`Canvas`][project::Canvas] type
#[derive(Clone)]
pub struct Canvas(pub Context<project::TrueCanvas, ()>);

impl<'lua> FromLua<'lua> for Canvas {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Canvas> {
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
        methods.document_type("A set of Layers with uniform dimensions and a Palette.");

        //Lua interface to from_layers() [also counts as interface for new()]
        /*
        {
            mlua_create_named_parameters!(
                CanvasArgs with
                    dimensions: PCoord,
                    layers: Vec<Layer>,
                    palette: Palette,
            );
            methods.document(
                "Creates & returns a new Canvas by providing its dimensions, an \
                             optional list of layers, and a Palette",
            );
            methods.add_meta_method(MetaMethod::Call, |_, _, a: CanvasArgs| {
                use mlua::Error::ExternalError;
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                let mut layers = Vec::new();
                for layer in a.layers {
                    layers.push(layer.0.do_imt(
                        |layer| Ok(layer.clone()),
                        |pixylene, index| pixylene.project.canvas
                            .get_layer(*index)
                            .map(|layer| layer.clone())
                    ).map_err(|_| ExternalError(Arc::from(boxed_error(LAYER_GONE))))?);
                }

                match project::Canvas::from_layers(
                    a.dimensions.0,
                    layers,
                    a.palette.0.do_imt(
                        |palette| palette.clone(),
                        |pixylene, _| pixylene.project.canvas.palette.clone()
                    )
                ) {
                    Ok(canvas) => Ok(Canvas(Context::Solo(canvas))),
                    Err(err) => Err(ExternalError(Arc::from(boxed_error(&err.to_string())))),
                }
            });
        }
        */

        {
            mlua_create_named_parameters!(
                CanvasArgs with
                    dimensions: PCoord,
            );
            methods.document(
                "Creates & returns a new true-color Canvas of the provided dimensions",
            );
            methods.add_meta_method(MetaMethod::Call, |_, _, a: CanvasArgs| {
                Ok(Canvas(Context::Solo(project::TrueCanvas::new(a.dimensions))))
            });
        }

        //Lua interface to merged_scene()
        {
            mlua_create_named_parameters!(
                CanvasMergeArgs with
                    background: Option<TruePixel>,
            );
            methods.document(
                "Merges the layers of the true-color Canvas into a Scene with an optional \
                background color",
            );
            methods.add_method("merge", |_, this, a: CanvasMergeArgs| {
                let color = match a.background {
                    Some(color) => Some(color.0),
                    None => None,
                };
                Ok(Scene(Context::Solo(this.0.do_imt(
                    |canvas| canvas.merged_scene(color),
                    |pixylene, _| pixylene.project.canvas.merged_scene(color),
                ))))
            });
        }

        //Lua interface to add_layer()
        {
            mlua_create_named_parameters!(
                CanvasAddArgs with
                    layer: Layer,
            );
            methods.document("Adds a Layer to the back of the Canvas");
            methods.add_method_mut("add", |_, this, a: CanvasAddArgs| {
                use mlua::Error::ExternalError;
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                let layer = a.layer.0.do_imt(
                    |layer| Ok(layer.clone()),
                    |pixylene, index| pixylene.project.canvas.get_layer(*index).map(|layer| layer.clone())
                ).map_err(|_| ExternalError(Arc::from(boxed_error(LAYER_GONE))))?;

                match &mut this.0 {
                    Context::Solo(ref mut canvas) => canvas.add_layer(layer),
                    Context::Linked(pixylene, _) => pixylene.borrow_mut().project.canvas.add_layer(layer),
                }.map_err(|err| ExternalError(Arc::from(boxed_error(&err.to_string()))))
            });
        }

        //Lua interface to get_layer_mut()
        {
            mlua_create_named_parameters!(
                CanvasGetArgs with
                    index: u16,
            );
            methods.document("Gets the Layer at the specified 1-based index in the Canvas");
            methods.add_method("layer", |_, this, a: CanvasGetArgs| {
                use Context::*;
                use mlua::Error::ExternalError;
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                Ok(Layer(
                    match &this.0 {
                        Solo(ref canvas) =>
                            canvas.get_layer(a.index.checked_sub(1).unwrap_or(0))
                            .map(|layer| Solo(layer.clone())),
                        Linked(pixylene, _) =>
                            pixylene.borrow_mut().project.canvas
                            .get_layer_mut(a.index.checked_sub(1).unwrap_or(0))
                            .map(|_| Linked(pixylene.clone(), a.index.checked_sub(1).unwrap_or(0))),
                    }.map_err(|err| ExternalError(Arc::from(boxed_error(&err.to_string()))))?
                ))
            });
        }

        //Hacky Lua interface to get_layer_mut()
        //{
        //    mlua_create_named_parameters!(
        //        CanvasSetArgs with
        //            index: u16,
        //            layer: Layer,
        //    );
        //    methods.document("Sets the Layer at the specified index in the Canvas");
        //    methods.add_method_mut("set", |_, this, a: CanvasSetArgs| {
        //        use mlua::Error::{ ExternalError };
        //        let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

        //        match this.0.get_layer_mut(a.index) {
        //            Ok(layer) => {
        //                *layer = a.layer.0;
        //                Ok(())
        //            },
        //            Err(err) => Err(ExternalError(Arc::from(
        //                boxed_error(&err.to_string())
        //            ))),
        //        }
        //    });
        //}

        //Lua interface to del_layer()
        //{
        //    mlua_create_named_parameters!(
        //        CanvasDelArgs with
        //            index: u16,
        //    );
        //    methods.document("Deletes and returns the Layer at the specified index in the Canvas");
        //    methods.add_method_mut("del", |_, this, a: CanvasDelArgs| {
        //        use mlua::Error::{ ExternalError };
        //        let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

        //        match this.0.del_layer(a.index) {
        //            Ok(layer) => Ok(Layer(layer)),
        //            Err(err) => Err(ExternalError(Arc::from(
        //                boxed_error(&err.to_string())
        //            ))),
        //        }
        //    });
        //}

        //Lua interface to duplicate_layer()
        //{
        //    mlua_create_named_parameters!(
        //        CanvasDuplicateArgs with
        //            index: u16,
        //    );
        //    methods.document("Duplicates the Layer at the specified index in the Canvas and \
        //                     places it at the next index");
        //    methods.add_method_mut("duplicate", |_, this, a: CanvasDuplicateArgs| {
        //        use mlua::Error::{ ExternalError };
        //        let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

        //        match this.0.duplicate_layer(a.index) {
        //            Ok(()) => Ok(()),
        //            Err(err) => Err(ExternalError(Arc::from(
        //                boxed_error(&err.to_string())
        //            ))),
        //        }
        //    });
        //}

        //Lua interface to move_layer()
        //{
        //    mlua_create_named_parameters!(
        //        CanvasMoveArgs with
        //            old_index: u16,
        //            new_index: u16,
        //    );
        //    methods.document("Move the Layer at the specified index to another index in the \
        //                     Canvas");
        //    methods.add_method_mut("move", |_, this, a: CanvasMoveArgs| {
        //        use mlua::Error::{ ExternalError };
        //        let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

        //        match this.0.move_layer(a.old_index, a.new_index) {
        //            Ok(()) => Ok(()),
        //            Err(err) => Err(ExternalError(Arc::from(
        //                boxed_error(&err.to_string())
        //            ))),
        //        }
        //    });
        //}

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        use Context::*;

        //Lua interface to dim()
        fields.document("the dimensions of this Canvas");
        fields.add_field_method_get("dim", |_, this| Ok(this.0.do_imt(
            |canvas| PCoord(canvas.dim()),
            |pixylene, _| PCoord(pixylene.project.canvas.dim())
        )));

        //Lua interface to num_layers()
        fields.document("the number of Layers currently in this Canvas");
        fields.add_field_method_get("num_layers", |_, this| Ok(this.0.do_imt(
            |canvas| canvas.num_layers(),
            |pixylene, _| pixylene.project.canvas.num_layers()
        )));

        //Lua interface to field palette
        fields.document("the palette composed by this Canvas");
        fields.add_field_method_get("palette", |_, this| Ok(match &this.0 {
            Solo(ref canvas) => Palette(Solo(canvas.palette.clone())),
            Linked(pixylene, _) => Palette(Linked(pixylene.clone(), ())),
        }));
        fields.add_field_method_set("palette", |_, this, palette: Palette| Ok(match &mut this.0 {
            Solo(ref mut canvas) => match palette.0 {
                Solo(palette) => {
                    canvas.palette = palette.clone();
                },
                Linked(pixylene2, _) => {
                    canvas.palette = pixylene2.borrow().project.canvas.palette.clone();
                },
            },
            Linked(pixylene, _) => match palette.0 {
                Solo(palette) => {
                    pixylene.borrow_mut().project.canvas.palette = palette.clone();
                },
                Linked(pixylene2, _) => {
                    pixylene.borrow_mut().project.canvas.palette =
                        pixylene2.borrow().project.canvas.palette.clone();
                },
            },
        }));
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
