extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod insertable;

#[proc_macro_derive(StructFragment, attributes(fragment_name, fragment_prelude, fragment_ignore, fragment_ignore_list))]
pub fn insertable_struct_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    insertable::impl_insertable_struct_macro(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // How test?! :( 
    }
}
