use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn register_theorem(attr: TokenStream, item: TokenStream) -> TokenStream {
    let category = parse_macro_input!(attr as LitStr);
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_ident = &input_fn.sig.ident;

    let init_name = syn::Ident::new(&format!("__init_{}", fn_ident), fn_ident.span());

    quote! {
        #input_fn

        #[::linkme::distributed_slice(crate::subjects::math::formalism::registry::THEOREM_REGISTRATIONS)]
        static __THEOREM_REGISTRATION: ::std::sync::OnceLock<crate::subjects::math::formalism::registry::TheoremMeta> =
            ::std::sync::OnceLock::new();

        #[::ctor::ctor]
        fn #init_name() {
            let theorem = #fn_ident();
            __THEOREM_REGISTRATION.get_or_init(|| crate::subjects::math::formalism::registry::TheoremMeta {
                theorem,
                category: #category.to_string(),
                module_path: module_path!().to_string(),
                file_path: file!().to_string(),
            });
        }
    }
    .into()
}
