// Tested in style4rs-test module...
//
use style4rs_util::{tokens_as_class_name, css_to_css_with_class_name};

use proc_macro2::LineColumn;
use syn::{ Macro, visit::Visit };
use time::{
    format_description::FormatItem,
    macros::*,
    OffsetDateTime
};
use walkdir::{WalkDir, DirEntry};

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Default)]
struct FileVisitor {
    class_styles: HashMap<String, String>,
    source: String,
}

impl<'ast> Visit<'ast> for FileVisitor {
    fn visit_macro(&mut self, node: &'ast Macro) {
        if let Some(ident) = node.path.get_ident() {
            if *ident == "style" || *ident == "style_str" {
                let tokens = &node.tokens;
                let class_name = tokens_as_class_name(tokens);
                let tokens = Vec::from_iter(tokens.clone());

                let (start, end) = 
                    if !tokens.is_empty() {
                        let len = tokens.len();
                        (tokens[0].span().start(), tokens[len-1].span().end())                       
                    } else {
                        panic!("Style4rsBuilder found invalid or empty macro");
                    };
                let css = self.extract_source(start, end);
                let css = css_to_css_with_class_name(&css, &class_name).unwrap();
                self.class_styles.insert(class_name, css);
            }
        }
    }
}

impl FileVisitor {
    fn new(source: &str) -> Self {
        FileVisitor { source: source.to_owned(), ..FileVisitor::default() }
    }

    fn visit(&mut self) {
        let ast = syn::parse_file(&self.source).unwrap();
        self.visit_file(&ast);
    }

    fn extract_source(&self, start: LineColumn, end: LineColumn) -> String {
        let mut lines: Vec<_> = self.source.lines()
            .skip(start.line-1)
            .take(end.line - start.line + 1)
            .collect();

        let end_index = lines.len() - 1;

        lines[end_index] = &lines[end_index][..end.column];
        lines[0] = &lines[0][start.column..];
    
        lines.join("\n")
    }
}

#[derive(Default)]
pub struct Style4rsBuilder {
    in_path: String,
    out_path: String,
    class_styles: HashMap<String, String>,
}


impl Style4rsBuilder {
    pub fn rs_to_css(&mut self, rs: &str) {
        let mut visitor = FileVisitor::new(rs);
        visitor.visit();
        for (key, value) in visitor.class_styles {
            self.class_styles.insert(key, value);
        }
    }

    fn rsfile_to_css(&mut self, entry: DirEntry) -> io::Result<()> {
        let path = entry.path();
        let source = fs::read_to_string(path).unwrap();
        self.rs_to_css(&source);
        Ok(())
    }

    fn is_rust_file(entry: &DirEntry) -> bool {
        let is_file = entry.file_type().is_file();
        let is_rs_extension = match entry.path().extension() {
            Some(ext) => ext == "rs",
            None => false,
        };
        is_file && is_rs_extension
    }

    fn rsfiles_to_css(&mut self, dir_path: &Path) -> io::Result<()> {
        let files_of_interest = WalkDir::new(dir_path).into_iter().flatten();
        let files_of_interest = files_of_interest.filter(Style4rsBuilder::is_rust_file);
        
        for entry in files_of_interest {
            _ = self.rsfile_to_css(entry);
        }
    
        Ok(())
    }

    fn extract_css_from_macros(&mut self) -> io::Result<String> {
        let source_path = Path::new(&self.in_path).join("src");

        self.rsfiles_to_css(&source_path).unwrap();

        let mut main_css = String::new();
        for (class_name, class_css) in self.class_styles.clone().into_iter() {
            main_css += &format!("\n/* {} */\n{}", class_name, class_css);
        }

        Ok(main_css)
    }

    fn write_to_main_css(&self, css: &String) -> io::Result<()> {
        let target_path = Path::new(&self.out_path).join("style4rs");
        fs::create_dir_all(&target_path).unwrap();
        let target_path = target_path
            .join("main.css");

        const FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day] [hour]:[minute] [[UTC[offset_hour sign:mandatory padding:zero]]");
        let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc()).format(FORMAT).unwrap();
        fs::write(target_path, format!(r"/*
 * This file was generated using Style4rsBuilder,
 * ANY EDITS MAY BE OVERWRITTEN.
 *
 * {}.
 */
{}", now, css))
    }

    pub fn build() -> io::Result<()> {
        Style4rsBuilder::build_using(None, None)
    }

    pub fn build_using(maybe_in_path: Option<String>, maybe_out_path: Option<String>) -> io::Result<()> {
        let in_path = maybe_in_path.or(env::var("CARGO_MANIFEST_DIR").ok()).unwrap();
        let out_path = maybe_out_path.or(env::var("OUT_DIR").ok()).unwrap();

        let mut builder = Style4rsBuilder { in_path, out_path, ..Style4rsBuilder::default() };
        let css = builder.extract_css_from_macros().unwrap();
        _ = builder.write_to_main_css(&css);
        Ok(())
    }

}
