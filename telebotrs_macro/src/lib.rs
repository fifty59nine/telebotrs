use proc_macro::TokenStream;
use quote::quote;
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
    println!("Attrs: {:?}", item_fn.attrs);
    let attr_name = &item_attr.path.segments.last().unwrap().ident.to_string();
    println!("Attr: {:?}\nNonP: {:?}", attr_name, item_attr);
    let fn_name = &item_fn.sig.ident;
    quote! {
        #fn_name();
    };
    TokenStream::new()
}


