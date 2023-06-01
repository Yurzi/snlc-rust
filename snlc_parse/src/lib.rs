use proc_macro::TokenStream;

#[proc_macro]
pub fn snl_expr(input: TokenStream) -> TokenStream {
    snlc_ast::expr::snl_expr(input.into()).into()
}

#[proc_macro]
pub fn snlc_expr(input: TokenStream) -> TokenStream {
    snlc_ast::expr::snlc_expr(input.into()).into()
}

#[proc_macro]
pub fn snl_stm(input: TokenStream) -> TokenStream {
    snlc_ast::stm::snl_stm(input.into()).into()
}

#[proc_macro]
pub fn snlc_stm(input: TokenStream) -> TokenStream {
    snlc_ast::stm::snlc_stm(input.into()).into()
}

#[proc_macro]
pub fn snl_prog(input: TokenStream) -> TokenStream {
    println!("{:?}", input);
    snlc_ast::ast::snl_program(input.into()).into()
}

#[proc_macro]
pub fn snlc_prog(input: TokenStream) -> TokenStream {
    snlc_ast::ast::snlc_program(input.into()).into()
}

#[proc_macro]
pub fn snl(input: TokenStream) -> TokenStream {
    snlc_ast::snl(input.into()).into()
}
