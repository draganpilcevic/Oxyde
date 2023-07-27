use syn::visit::{self, Visit};
use syn::{ItemFn, ImplItemFn};

struct FnVisitor<'ast> {
    functions: Vec<&'ast ItemFn>,
    methods: Vec<&'ast ImplItemFn>
}

impl<'ast> Visit<'ast> for FnVisitor<'ast> {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.functions.push(node);
        visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.methods.push(node);
        visit::visit_impl_item_fn(self, node);
    }
}

/*
fn main() {
    let code = quote! {
        pub fn f() {
            fn g() {}
        }
    };

    let syntax_tree: File = syn::parse2(code).unwrap();
    let mut visitor = FnVisitor { functions: Vec::new() };
    visitor.visit_file(&syntax_tree);
    for f in visitor.functions {
        println!("Function with name={}", f.sig.ident);
    }
}
*/