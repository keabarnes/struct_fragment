extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod fragment;

#[proc_macro_derive(StructFragment, attributes(fragment_name, fragment_prelude, fragment_ignore, fragment_ignore_list))]
pub fn struct_fragment_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    fragment::impl_struct_fragment_macro(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // How test?! :( 
    }
}
