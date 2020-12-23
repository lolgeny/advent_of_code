extern crate proc_macro;

use syn::{parse_macro_input, ItemFn, ReturnType, FnArg, Pat, Token};
use quote::quote;
use syn::punctuated::Punctuated;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

struct Args {
    args: Punctuated<Ident, Token![,]>
}
impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            args: input.parse_terminated(Ident::parse)?
        })
    }
}

#[proc_macro_attribute]
pub fn cache(attr: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let sig = f.sig.clone();
    let args = f.sig.inputs.iter().map(|arg| match arg {
        FnArg::Receiver(_) => panic!("Did not expect receiver function"),
        FnArg::Typed(arg) => arg
    });
    let arg_names = args.clone().map(|arg| arg.pat.clone());
    let names = quote! {
        #(#arg_names),*
    };
    let attr = parse_macro_input!(attr as Args);
    let attr_items = attr.args.iter();
    let attrs = quote! {
        #(#attr_items.clone()),*
    };
    let mut arg_types = vec![];
    for name in attr.args.iter() {
        arg_types.push(args.clone().filter(|arg| match *arg.pat {
            Pat::Ident(ref x) => x.ident == *name,
            _ => false
        }).next().expect("Unknown function parameter").ty.clone());
    }
    let types = quote! {
        #(#arg_types),*
    };
    let out = match f.sig.output {
        ReturnType::Default => panic!("Expected function to have return type"),
        ReturnType::Type(_, out) => *out
    };
    let body = f.block;
    let tokens = quote! {
        #sig {
            static mut __cache: ::std::option::Option<::std::collections::HashMap<(#types), #out>> =
            ::std::option::Option::None;
            let __cache_key = (#attrs);
            unsafe {
                match __cache {
                    ::std::option::Option::None => {
                        __cache = ::std::option::Option::Some(::std::collections::HashMap::new());
                    }
                    ::std::option::Option::Some(ref cache) => {
                        if cache.contains_key(&__cache_key) {
                            return cache.get(&__cache_key).unwrap().clone();
                        }
                    }
                }
            }
            fn __get_result(#(#args),*) -> #out {
                #body
            }
            let __result = __get_result(#names);
            unsafe {
                match __cache {
                    ::std::option::Option::Some(ref mut cache) => {cache.insert(__cache_key, __result);},
                    _ => {}
                }
            }
            __result
        }
    };
    // println!("{}", tokens);
    tokens.into()
}