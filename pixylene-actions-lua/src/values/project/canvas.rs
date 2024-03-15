use crate::values::{ types::{ PCoord, Pixel }, project::{ Scene, Layer, Palette } };

use tealr::{
    mlu::{
        mlua::{
            self,
            prelude::{ LuaValue },
            FromLua, Lua, Result, UserData, UserDataFields, UserDataMethods, MetaMethod,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody, mlua_create_named_parameters,
};
use std::sync::Arc;
use libpixylene::{ project };


/// Lua interface to libpixylene's [`Canvas`][project::Canvas] type
#[derive(Clone)]
pub struct Canvas(pub project::Canvas);

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
        {
            mlua_create_named_parameters!(
                CanvasArgs with
                    dimensions: PCoord,
                    layers: Vec<Layer>,
                    palette: Palette,
            );
            methods.document("Creates & returns a new Canvas by providing its dimensions, an \
                             optional list of layers, and a Palette");
            methods.add_meta_method(MetaMethod::Call, |_, _, a: CanvasArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match project::Canvas::from_layers(
                    a.dimensions.0,
                    a.layers.iter().map(|layer| layer.0.clone()).collect(),
                    a.palette.0.clone(),
                ) {
                    Ok(canvas) => Ok(Canvas(canvas)),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        //Lua interface to merged_scene()
        {
            mlua_create_named_parameters!(
                CanvasMergeArgs with
                    background: Option<Pixel>,
            );
            methods.document("Merges the layers of the Canvas into a Scene with an optional \
                             background color");
            methods.add_method("merge", |_, this, a: CanvasMergeArgs| {
                Ok(Scene(this.0.merged_scene(match a.background {
                    Some(color) => Some(color.0),
                    None => None
                })))
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
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.add_layer(a.layer.0.clone()) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        //Lua interface to get_layer()
        {
            mlua_create_named_parameters!(
                CanvasGetArgs with
                    index: u16,
            );
            methods.document("Gets the Layer at the specified index in the Canvas");
            methods.add_method("get", |_, this, a: CanvasGetArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.get_layer(a.index) {
                    Ok(layer) => Ok(Layer(layer.clone())),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        //Hacky Lua interface to get_layer_mut()
        {
            mlua_create_named_parameters!(
                CanvasSetArgs with
                    index: u16,
                    layer: Layer,
            );
            methods.document("Sets the Layer at the specified index in the Canvas");
            methods.add_method_mut("set", |_, this, a: CanvasSetArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.get_layer_mut(a.index) {
                    Ok(layer) => {
                        *layer = a.layer.0;
                        Ok(())
                    },
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        //Lua interface to del_layer()
        {
            mlua_create_named_parameters!(
                CanvasDelArgs with
                    index: u16,
            );
            methods.document("Deletes and returns the Layer at the specified index in the Canvas");
            methods.add_method_mut("del", |_, this, a: CanvasDelArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.del_layer(a.index) {
                    Ok(layer) => Ok(Layer(layer)),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        //Lua interface to duplicate_layer()
        {
            mlua_create_named_parameters!(
                CanvasDuplicateArgs with
                    index: u16,
            );
            methods.document("Duplicates the Layer at the specified index in the Canvas and \
                             places it at the next index");
            methods.add_method_mut("duplicate", |_, this, a: CanvasDuplicateArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.duplicate_layer(a.index) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }


        //Lua interface to move_layer()
        {
            mlua_create_named_parameters!(
                CanvasMoveArgs with
                    old_index: u16,
                    new_index: u16,
            );
            methods.document("Move the Layer at the specified index to another index in the \
                             Canvas");
            methods.add_method_mut("move", |_, this, a: CanvasMoveArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.move_layer(a.old_index, a.new_index) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        //Lua interface to dim()
        fields.document("the dimensions of this Canvas");
        fields.add_field_method_get("dim", |_, this| Ok(PCoord(this.0.dim())));

        //Lua interface to num_layers()
        fields.document("the number of Layers currently in this Canvas");
        fields.add_field_method_get("num_layers", |_, this| Ok(this.0.num_layers()));

        //Lua interface to field palette
        fields.document("the palette composed by this Canvas");
        fields.add_field_method_get("palette", |_, this| Ok(Palette(this.0.palette.clone())));
        fields.add_field_method_set("palette", |_, this, value: Palette| {
            this.0.palette = value.0;
            Ok(())
        });
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
