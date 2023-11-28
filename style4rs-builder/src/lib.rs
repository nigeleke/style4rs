use style4rs_util::as_class_name;

use proc_macro2::TokenStream;
use syn::Macro;
use syn::visit::{self, Visit};
use time::{format_description::FormatItem, macros::*, OffsetDateTime};
use walkdir::{WalkDir, DirEntry};

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Default)]
pub struct Style4rsBuilder {
    class_styles: HashMap<String, TokenStream>,
}

impl<'ast> Visit<'ast> for Style4rsBuilder {
    fn visit_macro(&mut self, node: &'ast Macro) {
        if let Some(ident) = node.path.get_ident() {
            if *ident == "style" {
                let tokens = &node.tokens;
                let class_name = as_class_name(tokens);
                self.class_styles.insert(class_name, tokens.clone());
            }
        }
        visit::visit_macro(self, node)
    }
}

impl Style4rsBuilder {

    pub fn rs_to_css(&mut self, rs: String) -> io::Result<()> {
        let ast = syn::parse_str(&rs).unwrap();
        self.visit_file(&ast);
        Ok(())
    }

    fn rsfile_to_css(&mut self, entry: DirEntry) -> io::Result<()> {
        let path = entry.path();
        let content = fs::read_to_string(path).unwrap();
        self.rs_to_css(content)
    }

    fn confirm_follow(entry: &DirEntry) -> bool {
        let is_dir = entry.file_type().is_dir();
        let is_file = entry.file_type().is_file();
        let is_rs_extension = match entry.path().extension() {
            Some(ext) => ext == "rs",
            None => false,
        };
        is_dir || (is_file && is_rs_extension)
    }

    fn rsfiles_to_css(&mut self, dir_path: &Path) -> io::Result<()> {
        let files_of_interest = WalkDir::new(dir_path)
            .into_iter()
            .filter_entry(Style4rsBuilder::confirm_follow);
        
        for entry in files_of_interest.flatten() {
            if entry.file_type().is_file() {
                _ = self.rsfile_to_css(entry);
            }
        }
    
        Ok(())
    }

    fn extract_css_from_macros(&mut self) -> io::Result<String> {
        let source_path = env::var_os("CARGO_MANIFEST_DIR").expect("Expected $env::CARGO_MANIFEST_DIR");
        let source_path = Path::new(&source_path).join("src");

        self.rsfiles_to_css(&source_path).unwrap();

        let mut main_css = String::new();
        for (class_name, class_css) in self.class_styles.clone().into_iter() {
            main_css += &format!("\n/* {} */\n{:?}", class_name, class_css);
        }

        Ok(main_css)
    }

    fn write_to_main_css(&self, css: &String) -> io::Result<()> {
        let target_path = env::var_os("OUT_DIR").expect("Expected $env::OUT_DIR");
        let target_path = Path::new(&target_path)
            .join("style4rs");
        fs::create_dir_all(&target_path).unwrap();
        let target_path = target_path
            .join("main.css");

        const FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day] [hour]:[minute] [[UTC[offset_hour sign:mandatory padding:zero]]");
        let now = OffsetDateTime::now_local().unwrap().format(FORMAT).unwrap();
        fs::write(target_path, format!(r"/*
 * This file was generated using Style4rsBuilder,
 * ANY EDITS MAY BE OVERWRITTEN.
 *
 * {}.
 */
{}", now, css))
    }

    pub fn build() -> io::Result<()> {
        let mut builder = Style4rsBuilder::default();
        let css = builder.extract_css_from_macros().unwrap();
        _ = builder.write_to_main_css(&css);
        Ok(())
    }

}
