
fn main() {
#[cfg(feature = "with-symbol-table")]
        {
            use std::env;
            use std::path::Path;

            let out_dir = env::var("O").unwrap();

            let symbols_path = Path::new(&out_dir).join("symbols.o");

            println!("cargo:rustc-link-search=native={}", out_dir);
            println!("cargo:rustc-link-arg={}", symbols_path.display());
        }

#[cfg(not(feature = "with-symbol-table"))]
        {
            // Optionally, print a message or handle the case where the file doesn't exist
            println!("cargo:warning=symbols.o not found, skipping linking.");
        }
}
