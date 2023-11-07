use proc_macro2::LineColumn;
use std::ops::Range;
use syn::visit::Visit;
use syn::Block;
use syn::Expr;
use syn::ItemFn;

/// Unsafe code ranges in a Rust file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnsafeRanges {
    pub blocks: Vec<Range<LineColumn>>,
    pub fns: Vec<Range<LineColumn>>,
    pub exprs: Vec<Range<LineColumn>>,
}

impl UnsafeRanges {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            fns: Vec::new(),
            exprs: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for UnsafeRanges {
    fn visit_block(&mut self, block: &'ast Block) {
        // If the block is unsafe, print its span
        for stmt in &block.stmts {
            if let syn::Stmt::Item(syn::Item::Fn(item_fn)) = stmt {
                if let Some(unsafe_token) = &item_fn.sig.unsafety {
                    self.blocks
                        .push(unsafe_token.span.start()..unsafe_token.span.end());
                }
            }
        }
        // Continue walking the tree
        syn::visit::visit_block(self, block);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        // If the function is unsafe, print its span
        if let Some(unsafe_token) = &i.sig.unsafety {
            self.fns
                .push(unsafe_token.span.start()..unsafe_token.span.end());
        }
        // Continue walking the tree
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // If the expression is unsafe, print its span
        if let syn::Expr::Unsafe(expr_unsafe) = expr {
            self.exprs
                .push(expr_unsafe.unsafe_token.span.start()..expr_unsafe.unsafe_token.span.end());
        }
        // Continue walking the tree
        syn::visit::visit_expr(self, expr);
    }
}

/// Returns unsafe code ranges in the given Rust code.
pub fn unsafe_ranges(rust_code: &str) -> Result<UnsafeRanges, syn::Error> {
    // Parse the Rust code into a syntax tree
    let syntax_tree = syn::parse_file(rust_code)?;

    // Create a visitor to find unsafe code
    let mut visitor = UnsafeRanges::new();

    // Walk the syntax tree
    visitor.visit_file(&syntax_tree);

    Ok(visitor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_spans() {
        let code = r#"
            fn main() {
                unsafe {
                    println!("Hello, world!");
                }
            }
        "#;

        let unsafe_spans = unsafe_ranges(code).unwrap();
        assert_eq!(unsafe_spans.exprs.len(), 1);
        assert_eq!(unsafe_spans.exprs[0].start.line, 3);
        assert_eq!(unsafe_spans.exprs[0].end.line, 3);
    }
}
