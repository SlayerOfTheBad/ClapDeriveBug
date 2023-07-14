use proc_macro::TokenStream;

#[proc_macro_derive(ExternalDerive, attributes(id))]
pub fn derive(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
