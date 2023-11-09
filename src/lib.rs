use proc_macro2::LineColumn;
use syn::ItemImpl;
use syn::ItemTrait;
use syn::spanned::Spanned;
use std::ops::Range;
use syn::visit::Visit;
use syn::Block;
use syn::Expr;
use syn::ItemFn;

/// Unsafe code ranges in a Rust file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnsafeRanges {
    pub ranges: Vec<Range<LineColumn>>,
}

impl UnsafeRanges {
    fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for UnsafeRanges {
    fn visit_block(&mut self, block: &'ast Block) {
        // If the block is unsafe, add to the list.
        for stmt in &block.stmts {
            if let syn::Stmt::Item(syn::Item::Fn(item_fn)) = stmt {
                if item_fn.sig.unsafety.is_some() {
                    let x = item_fn.span().start();
                    let y = item_fn.span().end();
                    self.ranges
                        .push(x..y);
                }
            }
        }
        // Continue walking the tree
        syn::visit::visit_block(self, block);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        // If the function is unsafe, add to the list.
        if i.sig.unsafety.is_some() {
            let x = i.span().start();
            let y = i.span().end();
            self.ranges
                .push(x..y);
        }
        // Continue walking the tree
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // If the expression is unsafe, add to the list.
        if let syn::Expr::Unsafe(expr_unsafe) = expr {            
            let x = expr_unsafe.span().start();
            let y = expr_unsafe.span().end();
            self.ranges
                .push(x..y);
        }
        // Continue walking the tree
        syn::visit::visit_expr(self, expr);
    }

    fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
        // If the trait is unsafe, add to the list.
        if i.unsafety.is_some() {
            let x = i.span().start();
            let y = i.span().end();
            self.ranges
                .push(x..y);
        }
        // Continue walking the tree
        syn::visit::visit_item_trait(self, i);        
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        // If the trait is unsafe, add to the list.
        if i.unsafety.is_some() {
            let x = i.span().start();
            let y = i.span().end();
            self.ranges
                .push(x..y);
        }
        // Continue walking the tree
        syn::visit::visit_item_impl(self, i);
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
        assert_eq!(unsafe_spans.ranges.len(), 1);
        assert_eq!(unsafe_spans.ranges[0].start.line, 3);
        assert_eq!(unsafe_spans.ranges[0].end.line, 5);
    }
}
