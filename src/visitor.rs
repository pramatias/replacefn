use syn::visit::{self, Visit};
use syn::{ItemFn, ItemImpl};
use proc_macro2;
use quote::ToTokens;

pub struct FoundFunction {
    pub name: String,
    pub tokens: proc_macro2::TokenStream,
}

pub struct FnVisitor {
    pub function_name: String,
    pub found_functions: Vec<FoundFunction>,
}

impl FnVisitor {
    pub fn new(function_name: &str) -> Self {
        FnVisitor {
            function_name: function_name.to_string(),
            found_functions: Vec::new(),
        }
    }

    pub fn check_function_name(&mut self, ident: &syn::Ident, tokens: &proc_macro2::TokenStream) {
        if ident.to_string() == self.function_name {
            let found_function = FoundFunction {
                name: ident.to_string(),
                tokens: tokens.clone(),
            };
            self.found_functions.push(found_function);
        }
    }

    pub fn get_found_functions(self) -> Vec<FoundFunction> {
        self.found_functions
    }
}

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        match node {
            syn::Item::Fn(item_fn) => {
                self.check_function_name(&item_fn.sig.ident, &quote::quote! { #item_fn.block });
            }
            syn::Item::Impl(item_impl) => {
                // Visit associated functions inside the impl block
                for item in &item_impl.items {
                    if let syn::ImplItem::Fn(impl_fn) = item {
                        self.check_function_name(&impl_fn.sig.ident, &quote::quote! { #impl_fn.block });
                    }
                }
            }
            syn::Item::Struct(item_struct) => {
                // Handle struct-specific logic here
                println!("Found struct:");
                println!("Name: {}", item_struct.ident);

                // Print information about struct fields
                for field in &item_struct.fields {
                    if let Some(ident) = &field.ident {
                        println!("Field Name: {}", ident);

                        // Provide a mutable reference to a TokenStream as an argument
                        let mut field_type_tokens = proc_macro2::TokenStream::new();
                        quote::ToTokens::to_tokens(&field.ty, &mut field_type_tokens);
                        println!("Field Type: {:#?}", field_type_tokens);
                    }
                }

                // Add more details about the struct if needed
            }
            _ => {
                // Handle other item types if needed
            }
        }

        // Delegate to the default impl to visit other items.
        visit::visit_item(self, node);
    }
}
