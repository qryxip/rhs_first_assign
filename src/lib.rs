extern crate proc_macro;

use if_chain::if_chain;
use proc_macro2::{LineColumn, Span};
use quote::quote;
use syn::spanned::Spanned as _;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_macro_input, parse_quote, Block, Expr, ExprAssignOp, Ident, ItemFn, Stmt};

#[proc_macro_attribute]
pub fn rhs_first_assign(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let mut input = parse_macro_input!(input as ItemFn);
    let mut stmts = input.block.stmts.clone();
    for stmt in &mut stmts {
        Visitor.visit_stmt_mut(stmt);
    }
    input.block = Box::new(Block {
        brace_token: input.block.brace_token,
        stmts,
    });
    quote!(#attr #input).into()
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if_chain! {
            if let Stmt::Expr(expr) | Stmt::Semi(expr, _) = stmt;
            if let Expr::AssignOp(ExprAssignOp {
                attrs,
                left,
                op,
                right,
            }) = expr;
            then {
                let LineColumn { line, column } = op.span().start();
                let rhs = format!("__rhs_first_assign_rhs_l{}_c{}", line, column);
                let rhs = Ident::new(&rhs, Span::call_site());
                *expr = parse_quote!(
                    #(#attrs)*
                    {
                        let #rhs = #right;
                        #left #op #rhs;
                    }
                );
            } else {
                visit_mut::visit_stmt_mut(self, stmt);
            }
        }
    }
}
