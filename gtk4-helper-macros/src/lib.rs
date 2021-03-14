extern crate proc_macro;
extern crate proc_macro2;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use anyhow::anyhow;

use std::collections::HashMap;
use syn::__private::TokenStream2;
use std::str::FromStr;

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

// /home/nico/.cargo/git/checkouts/gtk-rs-48ef14c1f17c79fb/4afd471/glib/src/subclass/mod.rs

fn get_min_max<T: FromStr>(field: &FieldData) -> anyhow::Result<(T, T)> {
    let attributes = field.attributes.get("param").expect("No attributes for param");
    let min = attributes.get("min").expect("No min value").parse::<T>().map_err(|_|anyhow!("Invalid min value"))?;
    let max = attributes.get("max").expect("No max value").parse::<T>().map_err(|_|anyhow!("Invalid max value"))?;
    Ok((min, max))
}

fn param_desc_for_field(field: &FieldData) -> TokenStream2 {
    let field_name = &field.name;
    match field.field_mapper.as_str() {
        "String" => {
            quote!(
                    glib::ParamSpec::string(
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
                    glib::ParamSpec::int(
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
                    glib::ParamSpec::float(
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
                    glib::ParamSpec::double(
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
                    glib::ParamSpec::int64(
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
                    glib::ParamSpec::boolean(
                        #field_name,
                        #field_name,
                        #field_name,
                        false,
                        glib::ParamFlags::READWRITE,
                    )
                )
        }
        t => panic!("Unsupported type: {}", t)
    }
}

#[proc_macro_derive(DataModel, attributes(param))]
pub fn data_model_meta(_: TokenStream) -> TokenStream {
    return quote!().into()
}

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let ty = &input.ident;
    let fields = get_fields(&input);

    let mut struct_data = vec![];
    let mut field_constants = vec![];
    let mut params_desc = vec![];
    for field in &fields {
        if !field.attributes.contains_key("param") {
            continue;
        }

        let field_ident = syn::Ident::new(&field.name,  proc_macro2::Span::call_site());
        let field_name = &field.name;

        struct_data.push(quote!(
            (&#field_name, &self.#field_ident)
        ));

        field_constants.push(quote!(
            const #field_ident: &'static str = #field_name;
        ));

        params_desc.push(param_desc_for_field(&field));
    }

    return quote! (
        #[derive(DataModel, Default)]
        #input

        mod imp {
            use super::*;
            #[derive(Default, DataModel)]
            pub struct #ty {
                pub __data: RefCell<HashMap<String, glib::Value>>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for #ty {
                const NAME: &'static str = stringify!(#ty);
                type Type = super::wrp::#ty;
                type ParentType = glib::Object;
                type Interfaces = ();
            }
        }

        impl ObjectImpl for imp::#ty {
            fn properties() -> &'static [glib::ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                    #ty::get_properties()
                });
                PROPERTIES.as_ref()
            }

            fn set_property(&self, _obj: &Self::Type, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
                self.__data.borrow_mut().insert(pspec.get_name().to_string(), value.to_owned());
            }

            fn get_property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
                self.__data.borrow().get(pspec.get_name()).map(|v| v.to_owned()).ok_or(()).clone().unwrap()
            }

            // fn constructed(&self, obj: &Self::Type) {
            //     self.parent_constructed(obj);
            // }
        }

        mod wrp {
            use super::*;
            glib::wrapper! {
                pub struct #ty(ObjectSubclass<imp::#ty>);
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

            pub fn to_object(&self) -> wrp::#ty {
                wrp::#ty::new(&[#(#struct_data),*])
            }

            pub fn static_type() -> glib::types::Type {
                wrp::#ty::static_type()
            }

            #(#field_constants)*
        }


    ).into()
}