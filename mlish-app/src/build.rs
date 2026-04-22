use quote::{ToTokens, TokenStreamExt, format_ident, quote};
use std::{env, fs, path::Path};

struct FontData {
    extension: String,
    name: String,
    file_name: String,
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get manifest dir");
    let base_path = Path::new(&manifest_dir);

    let font_dir_path = base_path.join("assets").join("fonts");
    let output_path = base_path.join("src").join("generated");

    let font_data_list = fetch_fonts(&font_dir_path);

    let generated_font_module_code = generate_font_module(&font_data_list);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(
        output_path.join("font.rs"),
        generated_font_module_code.to_token_stream().to_string(),
    )
    .expect("Failed to write generated file");
}

fn fetch_fonts(path: &Path) -> Vec<FontData> {
    fs::read_dir(path)
        .expect("Font directory missing")
        .flatten()
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|entry| {
            let file_name = entry.file_name().into_string().ok()?;
            let path = Path::new(&file_name);

            let name = path.file_stem()?.to_str()?.to_string();
            let extension = path.extension()?.to_str()?.to_string();

            Some(FontData {
                name,
                extension,
                file_name,
            })
        })
        .collect()
}

fn generate_font_module(fonts: &[FontData]) -> impl ToTokens {
    let mut font_constants = quote! {};
    let mut vec_push_statements = quote! {};

    for font in fonts {
        let const_name = format_ident!("{}", font.name.to_uppercase().replace("-", "_"));
        let rel_path = format!("../../assets/fonts/{}", font.file_name);
        let font_name = &font.name;
        let font_ext = &font.extension;

        font_constants.append_all(quote! {
            const #const_name: &'static [u8] = include_bytes!(#rel_path);
        });

        vec_push_statements.append_all(quote! {
            fonts.push(Font {
                name: #font_name.to_string(),
                font_data: #const_name,
                extension: #font_ext.to_string(),
            });
        });
    }

    quote! {
        use std::sync::LazyLock;

        #[derive(Debug, Clone)]
        pub struct Font {
            pub name: String,
            pub font_data: &'static [u8],
            pub extension: String,
        }

        #font_constants

        pub const FONTS: LazyLock<Vec<Font>> = LazyLock::new(|| get_fonts());

        fn get_fonts() -> Vec<Font> {
            let mut fonts = Vec::new();
            #vec_push_statements
            fonts
        }
    }
}
