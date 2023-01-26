use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

/// Attribute for handling messages
///
/// usage: `
/// #[message_handler(telebotrs::types::ContentType::Message)] // Can be any avalible ContentType
/// fn some_handler(msg: TextMessage) {}
///`
/// For more details, check README.md -> Examples
#[proc_macro_attribute]
pub fn message_handler(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    let fnitem = match syn::parse::<syn::ItemFn>(input.clone()) {
        Ok(item) => item,
        Err(err) => return compile_error(input, err),
    };

    match Handler::new(args, fnitem) {
        Ok(handler) => {
            //println!("{:#?}", handler);
            return TokenStream::new();
        }
        Err(err) => return compile_error(input, err),
    }
}

#[derive(Debug)]
struct Handler {
    /// Name of message_handler message
    name: syn::Ident,

    /// Args passed to message_handler macro.
    arg: Arg,

    /// itemFn for function which handling message
    fnitem: syn::ItemFn,
}

impl Handler {
    pub fn new(args: syn::AttributeArgs, fnitem: syn::ItemFn) -> syn::Result<Self> {
        let name = fnitem.sig.ident.clone();

        let arg = Arg::new(args)?;

        Ok(Self { name, arg, fnitem })
    }
}

#[derive(Debug)]
struct Arg {
    /// ContentType field in string: Message, Command, Document, etc.
    handling_type: String,

    /// If using command, here will be value: start, help, etc.
    handling_param: Option<String>,
}

impl Arg {
    pub fn new(arg: syn::AttributeArgs) -> syn::Result<Self> {
        if arg.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                format!("Invalid usage of message_handler. Expected at least one argument"),
            ));
        }
        if arg.len() != 1 {
            return Err(syn::Error::new(
                Span::call_site(),
                format!("Invalid usage of message_handler attribute. Expected just one argument"),
            ));
        }

        let handling_type: String;
        let handling_param: Option<String>;

        match &arg[0] {
            syn::NestedMeta::Meta(meta) => {
                match meta {
                    syn::Meta::List(list) => {
                        // We in handler with attr
                        handling_type = String::from("Command");
                        /*
                                                if list.nested.len() < 1 {
                                                    return Err(get_standart_error(Some(
                                                        "Invalid usage of message_handler attribute. Required at least 1 argument"
                                                            .to_string(),
                                                    )));
                                                }
                        */
                        let handling_param_lit = &list.nested[0];
                        match handling_param_lit {
                            syn::NestedMeta::Lit(lit) => match lit {
                                syn::Lit::Str(litstr) => {
                                    handling_param = Some(litstr.value());
                                }
                                _ => {
                                    return Err(get_standart_error(
                                            Some("Invalid usage of ContentType::Command. Expected string argument".to_string())
                                        ));
                                }
                            },
                            _ => {
                                return Err(get_standart_error(
                                    Some("Invalid usage of ContentType::Command. Expected string argument".to_string())
                                ));
                            }
                        }
                    }
                    syn::Meta::Path(path) => {
                        // Another handlers
                        handling_param = None;
                        handling_type = path.segments[1].ident.to_string();
                    }
                    _ => return Err(get_standart_error(None)),
                }
            }
            _ => return Err(get_standart_error(None)),
        }

        // arg valid check
        if let Some(param) = &handling_param {
            if param.trim().is_empty() {
                return Err(get_standart_error(Some(
                    "Argument should be at least 1 char.".to_string(),
                )));
            }
            if handling_type != String::from("Command") {
                return Err(get_standart_error(Some(
                    "You can't use arguments without ContentType::Command".to_string(),
                )));
            }
        } else {
            if handling_type == "Command" {
                return Err(get_standart_error(Some(
                    "You can't use ContentType::Command without arguments. For catch all commands use '*' attribute".to_string(),
                )));
            }
        }

        Ok(Arg {
            handling_type,
            handling_param,
        })
    }
}

fn get_standart_error(text: Option<String>) -> syn::Error {
    let msg = if let Some(text) = text {
        text
    } else {
        "Invalid usage of message_handler attribute. Expected #[message_handler(ContentType::*)]"
            .to_string()
    };
    syn::Error::new(Span::call_site(), msg)
}

/// Method for errors on parsing
fn compile_error(mut item: TokenStream, err: syn::Error) -> TokenStream {
    let compile_error = TokenStream::from(err.to_compile_error());
    item.extend(compile_error);
    item
}
