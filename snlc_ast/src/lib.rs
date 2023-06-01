use ast::Program;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;

pub mod ast;
pub mod expr;
pub mod stm;
pub mod token;

pub mod kw {
    syn::custom_keyword!(r#program);
    syn::custom_keyword!(r#begin);
    syn::custom_keyword!(r#end);
    syn::custom_keyword!(r#procedure);
    syn::custom_keyword!(r#return);

    syn::custom_keyword!(r#type);
    syn::custom_keyword!(r#var);

    syn::custom_keyword!(r#if);
    syn::custom_keyword!(r#then);
    syn::custom_keyword!(r#else);
    syn::custom_keyword!(r#fi);

    syn::custom_keyword!(r#while);
    syn::custom_keyword!(r#do);
    syn::custom_keyword!(r#endwh);

    syn::custom_keyword!(r#char);
    syn::custom_keyword!(r#integer);
    syn::custom_keyword!(r#record);
    syn::custom_keyword!(r#array);
    syn::custom_keyword!(r#of);

    syn::custom_keyword!(r#read);
    syn::custom_keyword!(r#write);
}
#[derive(Default)]
struct Errors {
    list: Vec<syn::Error>,
}

#[allow(dead_code)]
impl Errors {
    fn error(&mut self, span: Span, message: String) {
        self.list.push(syn::Error::new(span, message));
    }
}

pub fn snl(input: TokenStream) -> TokenStream {
    let (mut output, erros) = snl_with_erros(input);
    output.extend(erros.into_iter().map(|e| e.to_compile_error()));

    output
}

pub fn snl_with_erros(input: TokenStream) -> (TokenStream, Vec<syn::Error>) {
    let mut errors = Errors::default();

    let prog = match syn::parse2(input) {
        Ok(input) => input,
        Err(e) => {
            // This allows us to display errors at the proper span, while minimizing
            // unrelated errors caused by bailing out (and not generating code).
            errors.list.push(e);
            Program {
                name: syn::Ident::new("yurzi", Span::call_site()),
                var_defs: None,
                procedure_defs: None,
                body: Vec::new(),
            }
        }
    };

    let output = prog.to_token_stream();
    (output, errors.list)
}
