use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Fields, LitInt, Meta, NestedMeta, parse_macro_input};

// Helper function to check for a simple marker attribute like #[skip_path_field]
fn has_skip_attribute(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path.is_ident("skip_path_field"))
}

#[proc_macro_derive(PathIndex, attributes(skip_path_field))]
pub fn path_index_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Assuming the PathIndex trait and PathIndexExt are in this specific path
    // This might need to be adjusted based on your project structure or if you create a common utility crate.
    let path_index_trait_path =
        quote! { crate::subjects::math::formalism::proof::path_index::PathIndex };
    let path_index_ext_trait_path =
        quote! { crate::subjects::math::formalism::proof::path_index::PathIndexExt };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut expanded_find_by_path;
    let mut expanded_collect_all_paths;

    match &input.data {
        Data::Struct(data_struct) => {
            let mut find_arms = Vec::new();
            let mut collect_stmts = Vec::new();
            let mut field_idx_counter = 0;

            for field in data_struct.fields.iter() {
                if has_skip_attribute(&field.attrs) {
                    continue;
                }
                let field_name = field
                    .ident
                    .as_ref()
                    .expect("Struct fields must be named for PathIndex derive");
                let current_field_idx = LitInt::new(
                    &format!("{}", field_idx_counter),
                    proc_macro2::Span::call_site(),
                );

                find_arms.push(quote! {
                    #current_field_idx => self.#field_name.find_by_path(rest),
                });
                collect_stmts.push(quote! {
                    let mut field_path = base_path.clone();
                    field_path.push(#current_field_idx_counter);
                    result.extend(self.#field_name.collect_all_paths(field_path));
                });
                field_idx_counter += 1;
            }

            expanded_find_by_path = quote! {
                if path.is_empty() {
                    return Some(self as &dyn #path_index_trait_path);
                }
                let index = path[0];
                let rest = &path[1..];
                match index {
                    #(#find_arms)*
                    _ => None, // Index out of bounds
                }
            };
            expanded_collect_all_paths = quote! {
                let mut result = vec![(base_path.clone(), self as &dyn #path_index_trait_path)];
                #(#collect_stmts)*
                result
            };
        }
        Data::Enum(data_enum) => {
            let mut find_match_arms = Vec::new();
            let mut collect_match_arms = Vec::new();

            for variant in data_enum.variants.iter() {
                let variant_ident = &variant.ident;
                let mut field_idx_counter = 0;

                match &variant.fields {
                    Fields::Named(fields_named) => {
                        let field_bindings = fields_named
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().unwrap())
                            .collect::<Vec<_>>();
                        let mut find_variant_arms = Vec::new();
                        let mut collect_variant_stmts = Vec::new();

                        for field_binding in &field_bindings {
                            // Need to check skip_path_field on the original field definition if possible,
                            // For now, assume fields in enums are not skipped or handle via field name patterns if necessary.
                            // This is a simplification.
                            let current_field_idx = LitInt::new(
                                &format!("{}", field_idx_counter),
                                proc_macro2::Span::call_site(),
                            );
                            find_variant_arms.push(quote! {
                                #current_field_idx => #field_binding.find_by_path(rest),
                            });
                            collect_variant_stmts.push(quote! {
                                let mut field_path = current_variant_path.clone(); // Path up to this variant's field
                                field_path.push(#field_idx_counter);
                                result.extend(#field_binding.collect_all_paths(field_path));
                            });
                            field_idx_counter += 1;
                        }

                        find_match_arms.push(quote! {
                            #name::#variant_ident { #(#field_bindings),* , .. } => {
                                match index {
                                    #(#find_variant_arms)*
                                    _ => None,
                                }
                            }
                        });
                        collect_match_arms.push(quote! {
                            #name::#variant_ident { #(#field_bindings),* , .. } => {
                                let current_variant_path = base_path.clone();
                                #(#collect_variant_stmts)*
                            }
                        });
                    }
                    Fields::Unnamed(fields_unnamed) => {
                        let field_bindings = (0..fields_unnamed.unnamed.len())
                            .map(|i| format_ident!("field_{}", i))
                            .collect::<Vec<_>>();
                        let mut find_variant_arms = Vec::new();
                        let mut collect_variant_stmts = Vec::new();

                        for (i, field_binding) in field_bindings.iter().enumerate() {
                            let current_field_idx = LitInt::new(
                                &format!("{}", field_idx_counter),
                                proc_macro2::Span::call_site(),
                            );
                            find_variant_arms.push(quote! {
                                #current_field_idx => #field_binding.find_by_path(rest),
                            });
                            collect_variant_stmts.push(quote! {
                                let mut field_path = current_variant_path.clone();
                                field_path.push(#field_idx_counter);
                                result.extend(#field_binding.collect_all_paths(field_path));
                            });
                            field_idx_counter += 1;
                        }

                        find_match_arms.push(quote! {
                            #name::#variant_ident(#(#field_bindings),*) => {
                                match index {
                                    #(#find_variant_arms)*
                                    _ => None,
                                }
                            }
                        });
                        collect_match_arms.push(quote! {
                            #name::#variant_ident(#(#field_bindings),*) => {
                                let current_variant_path = base_path.clone();
                                #(#collect_variant_stmts)*
                            }
                        });
                    }
                    Fields::Unit => {
                        find_match_arms.push(quote! {
                            #name::#variant_ident => None,
                        });
                        collect_match_arms.push(quote! {
                            #name::#variant_ident => {},
                        });
                    }
                }
            }
            expanded_find_by_path = quote! {
                if path.is_empty() { return Some(self as &dyn #path_index_trait_path); }
                let index = path[0];
                let rest = &path[1..];
                match self {
                    #(#find_match_arms)*
                    _ => None, // Should not happen if all variants covered
                }
            };
            expanded_collect_all_paths = quote! {
                let mut result = vec![(base_path.clone(), self as &dyn #path_index_trait_path)];
                match self {
                    #(#collect_match_arms)*
                    _ => {}, // Should not happen if all variants covered
                }
                result
            };
        }
        _ => panic!("PathIndex derive only supports structs and enums."),
    }

    let expanded = quote! {
        impl #impl_generics #path_index_trait_path for #name #ty_generics #where_clause {
            fn find_by_path(&self, path: &[usize]) -> Option<&dyn #path_index_trait_path> {
                #expanded_find_by_path
            }

            fn collect_all_paths(&self, base_path: Vec<usize>) -> Vec<(Vec<usize>, &dyn #path_index_trait_path)> {
                #expanded_collect_all_paths
            }
        }
    };

    TokenStream::from(expanded)
}
