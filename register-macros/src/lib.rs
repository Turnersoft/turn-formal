use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, Token, parse::Parser, parse_macro_input, punctuated::Punctuated};

#[proc_macro_attribute]
pub fn register_theorem(attr: TokenStream, item: TokenStream) -> TokenStream {
    let categories = if !attr.is_empty() {
        let parser = Punctuated::<LitStr, Token![,]>::parse_terminated;
        match parser.parse(attr) {
            Ok(cats) => cats.into_iter().collect::<Vec<_>>(),
            Err(e) => {
                let error_msg = format!("Failed to parse categories: {}", e);
                return quote! {
                    compile_error!(#error_msg);
                }
                .into();
            }
        }
    } else {
        Vec::new()
    };

    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_ident = &input_fn.sig.ident;

    let static_name = syn::Ident::new(
        &format!(
            "__THEOREM_REGISTRATION_{}",
            fn_ident.to_string().to_uppercase()
        ),
        fn_ident.span(),
    );
    let init_name = syn::Ident::new(&format!("__init_{}", fn_ident), fn_ident.span());

    quote! {
        #input_fn

        #[::linkme::distributed_slice(crate::subjects::math::formalism::registry::THEOREM_REGISTRATIONS)]
        static #static_name: ::std::sync::OnceLock<crate::subjects::math::formalism::registry::TheoremMeta> =
            ::std::sync::OnceLock::new();

        #[::ctor::ctor]
        fn #init_name() {
            let theorem = #fn_ident();
            #static_name.get_or_init(|| crate::subjects::math::formalism::registry::TheoremMeta {
                theorem,
                categories: vec![#(#categories.to_string()),*],
                module_path: module_path!().to_string(),
                file_path: file!().to_string(),
            });
        }
    }
    .into()
}
