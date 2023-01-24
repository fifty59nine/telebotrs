use proc_macro::TokenStream;
use quote::{quote, ToTokens};
/*
use proc_macro_error::proc_macro_error;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::punctuated::Punctuated;
*/

#[proc_macro_attribute]
pub fn message_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let item_attr = syn::parse_macro_input!(attr as syn::ExprPath);
    let attr_name = &item_attr.path.segments.last().unwrap().ident.to_string();

    let fn_name = &item_fn.sig.ident;
    println!(
        "Message handler in {}() for {} with params: {:?}",
        &fn_name.to_string(),
        &attr_name,
        &item_fn.attrs
    );
    quote! {
        #fn_name();
    };
    TokenStream::new()
}
