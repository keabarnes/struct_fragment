use proc_macro2::TokenStream;
use syn::{Fields, Data, Meta, MetaNameValue, Lit};


fn struct_content(struct_name: &syn::Ident, fields: &syn::FieldsNamed, not_inserted_fields: &Option<Vec<String>>, struct_prelude: String) -> TokenStream {
    let mut new_fields: Vec<&syn::Field> = Vec::new();
    fields.named.iter().for_each(|field| {
        let field_name = &field.clone().ident.unwrap().to_string();
        let has_not_inserted_attr = field.attrs.iter().any(|attr| attr.parse_meta().unwrap().name() == "not_inserted");
        let in_not_inserted_list = not_inserted_fields.clone().unwrap_or_else(|| vec![]).contains(&field_name);
        if !in_not_inserted_list && !has_not_inserted_attr {
            new_fields.push(field);
        }
    });

    let prelud_ident: TokenStream = struct_prelude.parse::<TokenStream>().unwrap();

    quote!{
        #prelud_ident
        pub struct #struct_name{
            #(#new_fields),*
        }
    }
}

// Implement this using `let field_name = &field.ident.clone().unwrap();§` above
// impl InsertableThing {
//   pub fn from_db_structure(original: Artist) -> InsertableThing {
//     InsertableThing {
//       name: original.name,
//       image_link: original.image_link,
//       image_key: original.image_key,
//     }
//   }
// }

pub fn impl_insertable_struct_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let mut prefix: Option<String> = Some(String::from("Insertable"));
    let mut struct_prelude: String = String::from("#[derive(Debug)]");
    let mut not_inserted_fields: Option<Vec<String>> = None;

    for option in ast.attrs.iter() {
        match option.parse_meta().unwrap() {
            Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "prefix" => {
                if let Lit::Str(lit) = lit {
                    prefix = Some(lit.value());
                }
            },
            Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "insertable_prelude" => {
                if let Lit::Str(lit) = lit {
                    struct_prelude = lit.value();
                }
            },
            Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "insertable_postable_prelude" => {
                if let Lit::Str(lit) = lit {
                    struct_prelude = lit.value();
                }
            },
            Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "not_inserted_csv" => {
                if let Lit::Str(lit) = lit {
                    not_inserted_fields = Some(lit.value().split(',').map(|field| {
                        String::from(field.trim())
                    }).collect());
                }
            },
            _ => ()
        }
    }

    let name = &ast.ident;
    let concatenated = format!("{}{}", prefix.unwrap_or_default(), name);
    let struct_name = syn::Ident::new(&concatenated, name.span());

    let block = match ast.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields) => {
                struct_content(&struct_name, fields, &not_inserted_fields, struct_prelude)
            },
            _ => panic!("Unit structs cannot use derive(IteratorStruct)".to_owned()),
        },
        _ => panic!(
            "Only structs can use derive(InsertableStruct)".to_owned()
        ),
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
