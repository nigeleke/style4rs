use crate::error::Error;
use crate::result::Result;

use style4rs_util::{css_to_css_with_class_name, file_path_tokens_to_class_name_and_css, tokens_as_class_name};

use std::collections::HashMap;
use proc_macro2::LineColumn;
use syn::{ Macro, visit::Visit };


#[derive(Default)]
pub(crate) struct FileVisitor {
    class_styles: HashMap<String, String>,
    source: String,
}

impl<'ast> Visit<'ast> for FileVisitor {
    fn visit_macro(&mut self, node: &'ast Macro) {
        if let Some(ident) = node.path.get_ident() {
            if *ident == "style" {
                let tokens = &node.tokens;
                let class_name = tokens_as_class_name(tokens);
                let tokens = Vec::from_iter(tokens.clone());

                let (start, end) = 
                    if !tokens.is_empty() {
                        let len = tokens.len();
                        (tokens[0].span().start(), tokens[len-1].span().end())                       
                    } else {
                        panic!("Builder found invalid or empty macro");
                    };
                let css = self.extract_source(start, end);
                let css = css_to_css_with_class_name(&css, &class_name).unwrap();
                self.class_styles.insert(class_name, css);
            } else if *ident == "style_sheet" {
                let tokens = &node.tokens;
                let (class_name, css) = file_path_tokens_to_class_name_and_css(tokens).unwrap();
                let css = css_to_css_with_class_name(&css, &class_name).unwrap();
                self.class_styles.insert(class_name, css);
            }
        }
    }
}

impl FileVisitor {
    // Create a new file visitor to "file visit" the given source...
    //
    pub(crate) fn new(source: &str) -> Self {
        FileVisitor { source: source.to_owned(), ..FileVisitor::default() }
    }

    // Visit the source by parsing the 'rust' contents.
    //
    pub(crate) fn visit(&mut self) -> Result<()> {
        let ast = syn::parse_file(&self.source).map_err(Error::FailedToParseFile)?;
        self.visit_file(&ast);
        Ok(())
    }

    pub(crate) fn class_styles(&self) -> HashMap<String, String> {
        self.class_styles.clone()
    }

    // Extract the source code from the file source, which exists between the start  end line/column boundaries.
    //
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
