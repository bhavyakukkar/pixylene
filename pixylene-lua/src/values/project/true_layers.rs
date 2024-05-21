//todo: documentation

use crate::{
    utils::{CanvasMismatch, ContextExpired, CANVAS_MISMATCH_TRUE, LAYER_GONE, BOXED_ERROR},
    values::{
        project::TrueLayer,
        types::PCoord,
    },
    Context,
};

use libpixylene::{types, project};
use std::sync::Arc;
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, FromLua, Lua, MetaMethod, UserData, UserDataFields,
            UserDataMethods, Error::ExternalError,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};



#[derive(Clone)]
pub struct TrueLayers(pub Context<project::Layers<types::TruePixel>, ()>);

impl<'lua> FromLua<'lua> for TrueLayers {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> mlua::Result<TrueLayers> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<TrueLayers>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "TrueLayers",
                message: None,
            }),
        }
    }
}

impl TealData for TrueLayers {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A set of Layers with uniform dimensions and a Palette.");

        {
            mlua_create_named_parameters!(
                TrueLayersArgs with
                    dimensions: PCoord,
            );
            methods.document(
                "Creates & returns a new true-color TrueLayers of the provided dimensions",
            );
            methods.add_meta_method(MetaMethod::Call, |_, _, a: TrueLayersArgs| {
                Ok(TrueLayers(Context::Solo(project::Layers::<types::TruePixel>::new(a.dimensions.0))))
            });
        }

        //Lua interface to add_layer()
        {
            mlua_create_named_parameters!(
                TrueLayersAddArgs with
                    layer: TrueLayer,
            );
            methods.document("Adds a Layer to the back of the TrueLayers");
            methods.add_method_mut("add", |_, this, a: TrueLayersAddArgs| {
                let layer = a.layer.0.do_imt::<_, _, CanvasMismatch<ContextExpired<
                    project::Layer<types::TruePixel>
                >>>
                    (|layer| Ok(Ok(layer.clone())))
                    (|pixylene, index| pixylene.project.canvas.layers.to_true()
                        .map(|layers| layers.get_layer(*index)
                            .map(|layer| layer.clone())
                            .map_err(|_| ())))
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?;

                this.0.do_mut::<_, _, CanvasMismatch<Result<(), project::LayersError>>>
                    (|layers| Ok(layers.add_layer(layer.clone())))
                    (|mut pixylene, _| pixylene.project.canvas.layers.to_true_mut()
                        .map(|layers| layers.add_layer(layer.clone())))
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                    .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
            });
        }

        //Lua interface to get_layer_mut()
        {
            mlua_create_named_parameters!(
                TrueLayersGetArgs with
                    index: u16,
            );
            methods.document("Gets the Layer at the specified 1-based index in the TrueLayers");
            methods.add_method("get", |_, this, a: TrueLayersGetArgs| {
                use Context::*;
                Ok(TrueLayer(
                    match &this.0 {
                        Solo(ref layers) =>
                            Ok(layers.get_layer(a.index.checked_sub(1).unwrap_or(0))
                                .map(|layer| Solo(layer.clone()))),
                        Linked(pixylene, _) =>
                            pixylene.borrow_mut().project.canvas.layers.to_true_mut()
                                .map(|layers| layers.get_layer_mut(a.index.checked_sub(1).unwrap_or(0))
                                    .map(|_| Linked(
                                        pixylene.clone(),
                                        a.index.checked_sub(1).unwrap_or(0)
                                    ))),
                    }
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                    .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))?
                ))
            });
        }

        //Lua interface to del_layer()
        {
            mlua_create_named_parameters!(
                TrueLayersDeleteArgs with
                    index: u16,
            );
            methods.document("Deletes and returns the Layer at the specified index in the TrueLayers");
            methods.add_method_mut("delete", |_, this, a: TrueLayersDeleteArgs|
                Ok(TrueLayer(Context::Solo(
                    this.0.do_mut::<_, _, CanvasMismatch<
                        Result<project::Layer<types::TruePixel>, project::LayersError>
                    >>
                        (|layers| Ok(layers.del_layer(a.index)))
                        (|mut pixylene, _| pixylene.project.canvas.layers.to_true_mut()
                            .map(|layers| layers.del_layer(a.index)))
                        .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                        .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))?
                )))
            );
        }

        //Lua interface to duplicate_layer()
        {
            mlua_create_named_parameters!(
                TrueLayersDuplicateArgs with
                    index: u16,
            );
            methods.document("Duplicates the Layer at the specified index in the TrueLayers and \
                             places it at the next index");
            methods.add_method_mut("duplicate", |_, this, a: TrueLayersDuplicateArgs|
                this.0.do_mut::<_, _, CanvasMismatch<Result<(), project::LayersError>>>
                    (|layers| Ok(layers.duplicate_layer(a.index)))
                    (|mut pixylene, _| pixylene.project.canvas.layers.to_true_mut()
                        .map(|layers| layers.duplicate_layer(a.index)))
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                    .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
            );
        }

        //Lua interface to move_layer()
        {
            mlua_create_named_parameters!(
                TrueLayersMoveArgs with
                    old_index: u16,
                    new_index: u16,
            );
            methods.document("Move the Layer at the specified index to another index in the \
                             TrueLayers");
            methods.add_method_mut("move", |_, this, a: TrueLayersMoveArgs| {
                this.0.do_mut::<_, _, CanvasMismatch<Result<(), project::LayersError>>>
                    (|layers| Ok(layers.move_layer(a.old_index, a.new_index)))
                    (|mut pixylene, _| pixylene.project.canvas.layers.to_true_mut()
                        .map(|layers| layers.move_layer(a.new_index, a.old_index)))
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?
                    .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
            });
        }

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        //Lua interface to dim()
        fields.document("the dimensions of this TrueLayers");
        fields.add_field_method_get("dim", |_, this| Ok(PCoord(this.0.do_imt::<_, _,
            CanvasMismatch<types::PCoord>
        >
            (|layers| Ok(layers.dim()))
            (|pixylene, _| pixylene.project.canvas.layers.to_true()
                .map(|layers| layers.dim()))
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?)));

        //Lua interface to num_layers()
        fields.document("the number of Layers currently in this TrueLayers");
        fields.add_field_method_get("len", |_, this| Ok(this.0.do_imt::<_, _,
            CanvasMismatch<u16>
        >
            (|layers| Ok(layers.len()))
            (|pixylene, _| pixylene.project.canvas.layers.to_true()
                .map(|layers| layers.len()))
            .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_TRUE))))?));
    }
}

impl ToTypename for TrueLayers {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("TrueLayers", tealr::KindOfType::External)
    }
}

impl UserData for TrueLayers {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for TrueLayers {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
