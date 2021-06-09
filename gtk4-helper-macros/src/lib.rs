extern crate proc_macro;
extern crate proc_macro2;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use anyhow::anyhow;

use std::collections::HashMap;
use std::str::FromStr;
use syn::__private::TokenStream2;

const FIELD_ATTRIBUTE: &'static str = "field";

#[derive(Debug)]
struct FieldData {
    name: String,
    attributes: HashMap<String, HashMap<String, String>>,
    field_type: String,
    generic_type: Option<String>,
    field_mapper: String,
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>().join("::")
}

fn get_attr_list(field: &syn::Field) -> HashMap<String, HashMap<String, String>> {
    let mut attrs = HashMap::new();
    for attr in &field.attrs {
        let name = attr.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>().join("::");
        if let Ok(syn::Meta::List(l) ) = attr.parse_meta() {
            let kvp = get_str_lit_list(&l);
            attrs.insert(name, kvp);
        }else {
            attrs.insert(name, HashMap::new());
        }
    }
    attrs
}

fn get_str_lit_list(meta_list: &syn::MetaList) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for n in meta_list.nested.iter() {
        if let syn::NestedMeta::Meta(nk) = n {
            if let syn::Meta::NameValue(kv) = nk {
                if let syn::Lit::Str(ref s) = kv.lit {
                    let key = path_to_string(nk.path());
                    let val = s.value();
                    map.insert(key, val);
                }
            }
        }
    }

    return map;
}

fn get_fields(struct_data: &syn::ItemStruct) -> Vec<FieldData>{
    let mut fields = vec![];
    if let syn::Fields::Named(named_fields) = &struct_data.fields {
        for field in &named_fields.named {
            if let syn::Type::Path(fp) = &field.ty {
                let type_name = path_to_string(&fp.path);
                let attributes = get_attr_list(&field);
                let mut field_data = FieldData {
                    name: field.ident.to_owned().expect("Name is not defined").to_string(),
                    attributes,
                    field_type: type_name.clone(),
                    generic_type: None,
                    field_mapper: type_name,
                };

                if let Some(_) = &field.ident {
                    let option_type = path_args_to_string(&fp.path);
                    if option_type != "" {
                        field_data.field_mapper = option_type.clone();
                        field_data.generic_type = Some(option_type);
                    }
                }
                fields.push(field_data);
            }
        }
    }
    return fields;
}

fn path_args_to_string(path: &syn::Path) -> String {
    path.segments.iter().map(|s| args_to_string(&s.arguments)).collect::<Vec<String>>().join("::")
}

fn args_to_string(args: &syn::PathArguments) -> String {
    // The `<'a, T>` in `std::slice::iter<'a, T>`.
    if let syn::PathArguments::AngleBracketed(ab) = args {
        return ab.args.iter().map(|a| generic_arg_to_string(&a)).collect::<Vec<String>>().join(":")
    }
    return String::new();
}

fn generic_arg_to_string(ga: &syn::GenericArgument) -> String {
    if let syn::GenericArgument::Type( syn::Type::Path(t)) = ga {
        return path_to_string(&t.path);
    }
    return String::new();
}

// ManualPersonObjecte/nico/.cargo/git/checkouts/gtk-rs-48ef14c1f17c79fb/4afd471/glib/src/subclass/mod.rs

fn get_min_max<T: FromStr>(field: &FieldData) -> anyhow::Result<(T, T)> {
    let attributes = field.attributes.get(FIELD_ATTRIBUTE).expect("No attributes for param");
    let min = attributes.get("min").expect("No min value").parse::<T>().map_err(|_|anyhow!("Invalid min value"))?;
    let max = attributes.get("max").expect("No max value").parse::<T>().map_err(|_|anyhow!("Invalid max value"))?;
    Ok((min, max))
}

fn param_desc_for_field(field: &FieldData) -> TokenStream2 {
    let field_name = &field.name.replace("_", "-");
    match field.field_mapper.as_str() {
        "String" => {
            quote!(
                    glib::ParamSpec::new_string(
                        #field_name,
                        #field_name,
                        #field_name,
                        None,
                        glib::ParamFlags::READWRITE,
                    )
                )
        }
        "i32" => {
            let (min, max) = get_min_max::<i32>(&field).unwrap();
            quote!(
                    glib::ParamSpec::new_int(
                        #field_name,
                        #field_name,
                        #field_name,
                        #min,
                        #max,
                        0,
                        glib::ParamFlags::READWRITE,
                    )
                )
        }
        "f32" => {
            let (min, max) = get_min_max::<f32>(&field).unwrap();
            quote!(
                    glib::ParamSpec::new_float(
                        #field_name,
                        #field_name,
                        #field_name,
                        #min,
                        #max,
                        0,
                        glib::ParamFlags::READWRITE,
                    )
                )
        }
        "f64" => {
            let (min, max) = get_min_max::<f64>(&field).unwrap();
            quote!(
                    glib::ParamSpec::new_double(
                        #field_name,
                        #field_name,
                        #field_name,
                        #min,
                        #max,
                        0.0,
                        glib::ParamFlags::READWRITE,
                    )
                )
        }
        "i64" => {
            let (min, max) = get_min_max::<i64>(&field).unwrap();
            quote!(
                    glib::ParamSpec::new_int64(
                        #field_name,
                        #field_name,
                        #field_name,
                        #min,
                        #max,
                        0.0,
                        glib::ParamFlags::READWRITE,
                    )
                )
        }
        "bool" => {
            quote!(
                    glib::ParamSpec::new_boolean(
                        #field_name,
                        #field_name,
                        #field_name,
                        false,
                        glib::ParamFlags::READWRITE,
                    )
                )
        }
        _ => {
            let ty = syn::Ident::new(&field.field_mapper,  proc_macro2::Span::call_site());
            quote!(
                    glib::ParamSpec::new_object(
                        #field_name,
                        #field_name,
                        #field_name,
                        #ty::static_type(),
                        glib::ParamFlags::READWRITE,
                    )
                )
        }
    }
}

#[proc_macro_derive(DataModel, attributes(field))]
pub fn data_model_meta(_: TokenStream) -> TokenStream {
    return quote!().into()
}

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let ty = &input.ident;
    let fields = get_fields(&input);
    let imp_mod_ident = syn::Ident::new(&format!("imp_{}", ty.to_string()),  proc_macro2::Span::call_site());
    let wrp_mod_ident = syn::Ident::new(&format!("wrp_{}", ty.to_string()),  proc_macro2::Span::call_site());

    let mut struct_data = vec![];
    let mut field_constants = vec![];
    let mut params_desc = vec![];
    let mut property_setter = vec![];

    for field in &fields {
        let field_ident = syn::Ident::new(&field.name,  proc_macro2::Span::call_site());

        if !field.attributes.contains_key(FIELD_ATTRIBUTE) {
            property_setter.push(quote!(
                #field_ident: Default::default()
            ));
            continue;
        }
        let field_name = &field.name.replace("_", "-");
        let field_type = syn::Ident::new(&field.field_mapper, proc_macro2::Span::call_site());

        let optional = field.field_type == "Option";
        if optional {
            property_setter.push(quote!(
                #field_ident: obj.property(#field_name).expect("No Property").get::<#field_type>().expect("Property type mismatch")
            ));
        }else {
            property_setter.push(quote!(
                #field_ident: obj.property(#field_name).expect("No Property").get::<#field_type>().expect("Property type mismatch")
            ));
        }


        struct_data.push(quote!(
            (&#field_name, &self.#field_ident)
        ));

        field_constants.push(quote!(
            pub const #field_ident: &'static str = #field_name;
        ));

        params_desc.push(param_desc_for_field(&field));
    }

    return quote! (
        #[derive(DataModel, Default, Debug)]
        #input

        mod #imp_mod_ident {
            use super::*;
            #[derive(Default, DataModel)]
            pub struct #ty {
                pub data: RefCell<HashMap<String, glib::Value>>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for #ty {
                const NAME: &'static str = stringify!(#ty);
                type Type = super::#wrp_mod_ident::#ty;
                type ParentType = glib::Object;
                type Interfaces = ();
            }
        }

        impl ObjectImpl for #imp_mod_ident::#ty {
            fn properties() -> &'static [glib::ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                    #ty::get_properties()
                });
                PROPERTIES.as_ref()
            }

            fn set_property(&self, _obj: &Self::Type, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
                self.data.borrow_mut().insert(pspec.name().to_string(), value.to_owned());
            }

            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
                self.data.borrow().get(pspec.name()).map(|v| v.to_owned()).ok_or(()).clone().unwrap()
            }
        }

        mod #wrp_mod_ident {
            use super::*;
            glib::wrapper! {
                pub struct #ty(ObjectSubclass<#imp_mod_ident::#ty>);
            }

            impl #ty {
                pub fn new(properties: &[(&str, &dyn ToValue)]) -> #ty {
                    glib::Object::new(properties).unwrap()
                }
            }
        }

        impl #ty {
            pub fn get_properties() -> Vec<ParamSpec> {
                vec![#(#params_desc),*]
            }

            pub fn new_object(properties: &[(&str, &dyn ToValue)]) -> glib::Object {
                #wrp_mod_ident::#ty::new(properties).upcast::<glib::Object>()
            }

            pub fn to_object(&self) -> glib::Object {
                #wrp_mod_ident::#ty::new(&[#(#struct_data),*]).upcast::<glib::Object>()
            }

            pub fn static_type() -> glib::types::Type {
                #wrp_mod_ident::#ty::static_type()
            }

            pub fn from_object(obj: &glib::Object) -> #ty {
                Self {
                    #(#property_setter),*
                }
            }

            #(#field_constants)*
        }

        impl From<glib::Object> for #ty {
            fn from(obj: Object) -> Self {
                #ty::from_object(&obj)
            }
        }

        impl From<#ty> for glib::Object {
            fn from(o: #ty) -> Self {
                #ty::to_object(&o)
            }
        }

        impl ToValue for #ty {
            fn to_value(&self) -> Value {
                self.to_object().to_value()
            }
            fn value_type(&self) -> Type {
                #ty::static_type()
            }
        }

        unsafe impl<'a> FromValue<'a> for #ty {
            type Checker = GenericValueTypeOrNoneChecker<#ty>;

            unsafe fn from_value(value: &'a Value) -> Self {
                value.get::<glib::Object>().and_then(|o| Ok(#ty::from_object(&o))).unwrap()
            }
        }

        impl StaticType for #ty {
            fn static_type() -> Type {
                #ty::static_type()
            }
        }

    ).into()
}