use syn::visit::{self, Visit};
use syn::{ItemFn, ItemImpl};
use proc_macro2;

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
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.check_function_name(&node.sig.ident, &quote::quote! { #node.block });
        // Delegate to the default impl to visit any nested functions.
        visit::visit_item_fn(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        // Visit associated functions inside the impl block
        for item in &node.items {
            if let syn::ImplItem::Fn(impl_fn) = item {
                self.check_function_name(&impl_fn.sig.ident, &quote::quote! { #impl_fn.block });
            }
        }

        // Delegate to the default impl to visit other items inside the impl block.
        visit::visit_item_impl(self, node);
    }

    // Handle other items if needed
    fn visit_item(&mut self, node: &'ast syn::Item) {
        // Implement custom logic for other types of items if needed.
        // For example, you can check if it's an impl block and visit its associated functions.
        // This depends on your specific requirements.
        visit::visit_item(self, node);
    }
}
