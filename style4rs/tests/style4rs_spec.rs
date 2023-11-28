use style4rs::style;
use style4rs_util::as_class_name;

use syn::Macro;
use syn::visit::{self, Visit};

use std::env;
use std::fs;
use std::path::Path;

#[test]
fn test_macro_hashes_are_equal_to_syn_parsed_hashes() {
    let macro_class_name = style!{
        div.one {
            color: red;
            content: "hello";
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
    };

    #[derive(Default)]
    struct Visitor {
        class_names: Vec<String>,
    }

    impl<'ast> Visit<'ast> for Visitor {
        fn visit_macro(&mut self, node: &'ast Macro) {
            if node.path.get_ident().unwrap().to_string() == "style" {
                let tokens = &node.tokens;
                self.class_names.push(as_class_name(tokens));
            }
            visit::visit_macro(self, node)
        }
    }
    
    let source_path = env::var_os("CARGO_MANIFEST_DIR").expect("Expected $env::CARGO_MANIFEST_DIR");
    let source_path = Path::new(&source_path).join("tests").join("style4rs_spec.rs");

    let content = fs::read_to_string(source_path).unwrap();
    let ast = syn::parse_str(&content).unwrap();

    let mut visitor = Visitor::default();
    visitor.visit_file(&ast);

    assert!(visitor.class_names.contains(&macro_class_name.to_string()));
}
