extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::{TokenStream};
use syn::{Fields, Data};

fn struct_content(struct_name: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
    let mut new_fields: Vec<&syn::Field> = Vec::new();
    fields.named.iter().for_each(|field| {
        let field_name = &field.ident.clone().unwrap();
        let illegal_field: &syn::Ident = &syn::Ident::new("id", proc_macro2::Span::call_site());
        if field_name != illegal_field {
            &new_fields.push(field);
        }
    });

    quote!{
        #[derive(Debug)]
        pub struct #struct_name{
            #(#new_fields),*
        }
    }
}

fn impl_insertabl_struct_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let concatenated = format!("Insertable{}", name);
    let struct_name = syn::Ident::new(&concatenated, name.span());

    let block = match ast.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields) => {
                struct_content(&struct_name, fields)
            },
            _ => panic!(format!("Unit structs cannot use derive(IteratorStruct)")),
        },
        _ => panic!(format!(
            "Only structs can use derive(InsertableStruct)",
        )),
    };

    block.into()
}

#[proc_macro_derive(InsertableStruct)]
pub fn insertable_struct_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_insertabl_struct_macro(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // How test?! :( 
    }
}
