extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn get_pkg_file(args: TokenStream) -> TokenStream {
    format!(
        r#"
        {{
            use std::path::PathBuf;
            let directory = env!("CARGO_MANIFEST_DIR");
            let mut builder = PathBuf::from(directory);
            builder.push({args});

            builder
        }}
    "#
    )
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn get_pkg_fixture(args: TokenStream) -> TokenStream {
    format!(
        r#"
        {{
            use std::path::PathBuf;
            use std::fs::read_to_string;
            let directory = env!("CARGO_MANIFEST_DIR");
            let mut builder = PathBuf::from(directory);
            builder.push("tests/fixtures");
            builder.push({args});

            read_to_string(builder)
        }}
    "#
    )
    .parse()
    .unwrap()
}
