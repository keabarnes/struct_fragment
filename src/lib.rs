extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod insertable;
mod postable;

#[proc_macro_derive(InsertableStruct, attributes(prefix, insertable_prelude, insertable_postable_prelude, not_inserted, not_inserted_csv))]
pub fn insertable_struct_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    insertable::impl_insertable_struct_macro(&ast)
}

#[proc_macro_derive(PostableStruct, attributes(prefix, postable_prelude, insertable_postable_prelude, not_posted, not_posted_csv))]
pub fn postable_struct_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    postable::impl_postable_struct_macro(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // How test?! :( 
    }
}
