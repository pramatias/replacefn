use std::error::Error;
use std::fs;
use syn::parse_str;
use syn::File;
use syn::visit::Visit;
use visitor::{FnVisitor};

mod visitor;

fn main() -> Result<(), Box<dyn Error>> {
    // Replace "lib.rs" with the path to your Rust source file
    let file_path = "lib.rs";
    let source_code = fs::read_to_string(file_path)?;

    // Parse the source code into a syn AST
    let syntax_tree: File = parse_str(&source_code)?;

    // Function name to search for
    //let function_name_to_search = "cargo_target_directory";
    let function_name_to_search = "plotting_backend";

    // Create an instance of the FnVisitor with the specified function name
    let mut fn_visitor = FnVisitor::new(function_name_to_search);

    // Visit the syntax tree
    fn_visitor.visit_file(&syntax_tree);

    // Get and print the found functions
    let found_functions = fn_visitor.get_found_functions();
    for found_function in found_functions {
        println!("Found function '{}'", found_function.name);
        println!("Tokens in {} function: {}", found_function.name, found_function.tokens);
    }

    Ok(())
}
