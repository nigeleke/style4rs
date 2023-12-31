use crate::error::Error;
use crate::file_visitor::FileVisitor;
use crate::result::Result;

use time::{
    format_description::FormatItem,
    macros::*,
    OffsetDateTime
};
use walkdir::{WalkDir, DirEntry};

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Builder {
    maybe_in_folder: Option<PathBuf>,
    maybe_out_file: Option<PathBuf>,
    class_styles: HashMap<String, String>,
}

impl Builder {
    fn rs_to_css(&mut self, rs: &str) -> Result<()> {
        let mut visitor = FileVisitor::new(rs);
        visitor.visit()?;
        for (key, value) in visitor.class_styles() {
            self.class_styles.insert(key, value);
        }
        Ok(())
    }

    fn rsfile_to_css(&mut self, entry: DirEntry) -> Result<()> {
        let path = entry.path();
        let source = fs::read_to_string(path).map_err(Error::FailedToReadFile)?;
        self.rs_to_css(&source)
    }

    fn is_rust_file(entry: &DirEntry) -> bool {
        let is_file = entry.file_type().is_file();
        let is_rs_extension = match entry.path().extension() {
            Some(ext) => ext == "rs",
            None => false,
        };
        is_file && is_rs_extension
    }

    fn rsfiles_to_css(&mut self) -> Result<()> {
        let dir_path = self.maybe_in_folder.as_mut().ok_or(Error::InFolderPathNotDefined)?;
        let files_of_interest = WalkDir::new(dir_path).into_iter().flatten();
        let files_of_interest = files_of_interest.filter(Builder::is_rust_file);
        
        for entry in files_of_interest {
            _ = self.rsfile_to_css(entry);
        }
        Ok(())    
    }

    fn extract_css_from_macros(&mut self) -> Result<String> {
        self.rsfiles_to_css()?;

        let mut main_css = String::new();
        for (class_name, class_css) in self.class_styles.clone().into_iter() {
            main_css += &format!("\n/* {} */\n{}", class_name, class_css);
        }

        Ok(main_css)
    }

    fn write_to_main_css(&self, css: &String) -> Result<()> {
        let target_file = self.maybe_out_file.as_ref().ok_or(Error::OutFilePathNotDefined)?;
        let target_folder = target_file.parent().unwrap();
        fs::create_dir_all(target_folder).map_err(Error::FailedToCreateTargetFolder)?;

        const FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day] [hour]:[minute] [[UTC[offset_hour sign:mandatory padding:zero]]");
        let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc()).format(FORMAT).unwrap();
        fs::write(target_file, format!(r"/*
 * This file was generated using Builder,
 * ANY EDITS MAY BE OVERWRITTEN.
 *
 * {}.
 */
{}", now, css)).map_err(Error::FailedToWriteTargetFile)?;
        Ok(())    
    }

    /// Create a new `Builder`.
    /// The default in-folder-path will be `$env:CARGO_MANIFEST_DIR/src`. This can be overidden with the `using_in_folder(...)` method. If 
    /// `CARGO_MANIFEST_DIR` is not defined then the user must supply the input folder.
    /// The default out-file-path will be `$env:OUT_DIR/style4rs/main.css`. This can be overidden with the `using_out_file(...)` method. If
    /// `OUT_DIR` is not defined then the user must supply the out file path.
    ///
    pub fn new() -> Self {
        let maybe_in_folder = env::var("CARGO_MANIFEST_DIR").map(|p| Path::new(&p).join("src")).ok();
        let maybe_out_file = env::var("OUT_DIR").map(|p| Path::new(&p).join("style4rs").join("main.css")).ok();
        Builder { maybe_in_folder, maybe_out_file, ..Builder::default() }
    }

    /// Override the default in folder with that provided. If the folder does not exist when `build()` is invoked the builder will panic.
    ///
    pub fn using_in_folder(&mut self, in_folder: &Path) -> &mut Self {
        self.maybe_in_folder = Some(in_folder.to_path_buf());
        self
    }

    /// Override the default out file with that provided. If the file exists then it will be overwritten. The folder path will be created
    /// if necessary.
    ///
    pub fn using_out_file(&mut self, out_file: &Path) -> &mut Self {
        self.maybe_out_file = Some(out_file.to_path_buf());
        self
    }

    /// Build the `style4rs::style` and `style4rs::style_sheet` macro invocation in all `.rs` files into the required output file.
    /// The output file will contain all css with component classes inserted appropriately.
    /// `build()` will panic if the input folder is not provided (or exists), or the outpur file path is not provided.
    ///
    pub fn build(&mut self) -> Result<()> {
        let css = self.extract_css_from_macros()?;
        _ = self.write_to_main_css(&css);
        Ok(())
    }

}
