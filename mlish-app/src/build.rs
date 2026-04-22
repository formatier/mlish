use std::{env, fs, path::PathBuf};

use quote::{TokenStreamExt, format_ident, quote};

struct FontData {
    extension: String,
    name: String,
    file_name: String,
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut project_src_dir_path = PathBuf::from(manifest_dir.clone());
    project_src_dir_path.push("src");
    project_src_dir_path.push("generated");

    let mut font_dir_path = PathBuf::from(manifest_dir.clone());
    font_dir_path.push("assets");
    font_dir_path.push("fonts");

    let font_dir = fs::read_dir(&font_dir_path).unwrap();
    let font_data_list = font_dir.filter_map(|font_dir_item| {
        let font_dir_item = font_dir_item.unwrap();
        let font_type = font_dir_item.file_type().unwrap();
        let font_name = font_dir_item.file_name().into_string().unwrap().to_string();

        if font_type.is_file() {
            let (font_name, font_extension) = font_name.split_once(".").unwrap();
            Some(FontData {
                extension: font_extension.to_string(),
                name: font_name.to_string(),
                file_name: format!("{}.{}", font_name.to_string(), font_extension.to_string()),
            })
        } else {
            None
        }
    });

    let mut font_tokens_main = quote! {
        pub struct Font{
            pub name: String,
            pub font_data: &'static [u8],
            pub extension: String
        }
    };

    let mut get_font_function_tokens = quote! {
        let mut font = Vec::new();
    };
    font_data_list.for_each(|font_data| {
        let font_name = font_data.name;
        let font_extension = font_data.extension;
        let font_path = format!("../../assets/fonts/{}", font_data.file_name);

        let font_name_token = format_ident!("{}", font_name.to_uppercase().replace("-", "_"));
        font_tokens_main.append_all(quote! {
            const #font_name_token: &'static [u8] = include_bytes!(#font_path);
        });

        get_font_function_tokens.append_all(quote! {
            font.push(Font{
                name: #font_name.to_string(),
                font_data: #font_name_token,
                extension: #font_extension.to_string()
            });
        });
    });

    font_tokens_main.append_all(quote! {
        pub fn get_font() -> Vec<Font> {
            #get_font_function_tokens
        }
    });

    let mut font_module_path = project_src_dir_path;
    font_module_path.push("font.rs");
    fs::write(font_module_path, font_tokens_main.to_string()).unwrap();
}
