use std::env;
use std::path::Path;

fn main() {
    // Get the output directory from the environment variables
    let out_dir = env::var("O").unwrap();

    // Define the path to the symbols.o file
    let symbols_path = Path::new(&out_dir).join("symbols.o");

    // Check if the symbols.o file exists
    if symbols_path.exists() {
        // If it exists, print the link search path and link the static library
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-arg={}", symbols_path.display());
    } else {
        // Optionally, print a message or handle the case where the file doesn't exist
        println!("cargo:warning=symbols.o not found, skipping linking.");
    }
}
