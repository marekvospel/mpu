extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn get_pkg_file(args: TokenStream) -> TokenStream {
    format!(r#"
        {{
            use std::path::PathBuf;
            let directory = env!("CARGO_MANIFEST_DIR");
            let mut builder = PathBuf::from(directory);
            builder.push({});

            builder
        }}
    "#, args.to_string()).parse().unwrap()
}

#[proc_macro]
pub fn get_pkg_fixture(args: TokenStream) -> TokenStream {
    format!(r#"
        {{
            use std::path::PathBuf;
            use std::fs::read_to_string;
            let directory = env!("CARGO_MANIFEST_DIR");
            let mut builder = PathBuf::from(directory);
            builder.push("tests/fixtures");
            builder.push({});

            read_to_string(builder)
        }}
    "#, args.to_string()).parse().unwrap()
}
