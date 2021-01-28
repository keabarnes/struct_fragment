use proc_macro2::TokenStream;
use syn::{Data, Fields, Lit, Meta, MetaNameValue};

fn struct_content(
    struct_name: &syn::Ident,
    fields: &syn::FieldsNamed,
    ignore_list: &Option<Vec<String>>,
    struct_prelude: String,
) -> TokenStream {
    let mut new_fields: Vec<&syn::Field> = Vec::new();
    fields.named.iter().for_each(|field| {
        let field_name = &field.clone().ident.unwrap().to_string();
        let has_ignore_attr = field.attrs.iter().any(|attr| {
            attr.parse_meta()
                .unwrap()
                .path()
                .is_ident("fragment_ignore")
        });
        let in_ignore_list = ignore_list
            .clone()
            .unwrap_or_else(Vec::new)
            .contains(&field_name);
        if !in_ignore_list && !has_ignore_attr {
            new_fields.push(field);
        }
    });

    let prelud_ident: TokenStream = struct_prelude.parse::<TokenStream>().unwrap();

    quote! {
        #prelud_ident
        pub struct #struct_name{
            #(#new_fields),*
        }
    }
}

pub fn impl_struct_fragment_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let original_struct_name = &ast.ident;
    let mut fragment_name: String = format!("{}Fragment", original_struct_name);
    let mut struct_prelude: String = String::from("#[derive(Debug)]");
    let mut ignore_list: Option<Vec<String>> = None;

    for attr in ast.attrs.iter() {
        // if attr.path.is_ident("fragment_name") {
        // } else if attr.path.is_ident("fragment_prelude") {
        // } else if attr.path.is_ident("fragment_ignore_list") {
        // }

        match attr.parse_meta().unwrap() {
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("fragment_name") => {
                if let Lit::Str(lit) = lit {
                    fragment_name = lit.value();
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("fragment_prelude") => {
                if let Lit::Str(lit) = lit {
                    struct_prelude = lit.value();
                }
            }
            Meta::NameValue(MetaNameValue {
                ref path, ref lit, ..
            }) if path.is_ident("fragment_ignore_list") => {
                if let Lit::Str(lit) = lit {
                    ignore_list = Some(
                        lit.value()
                            .split(',')
                            .map(|field| String::from(field.trim()))
                            .collect(),
                    );
                }
            }
            _ => (),
        }
    }

    let new_struct_name = syn::Ident::new(&fragment_name, original_struct_name.span());

    let block = match ast.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields) => {
                struct_content(&new_struct_name, fields, &ignore_list, struct_prelude)
            }
            _ => panic!("Unit structs cannot use derive(IteratorStruct)".to_owned()),
        },
        _ => panic!("Only structs can use derive(InsertableStruct)".to_owned()),
    };

    block.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // How test?! :(
    }
}
