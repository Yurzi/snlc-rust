use crate::expr::Expr;
use crate::kw;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parenthesized, Token};
use syn::{parse::Parse, parse::ParseStream};

pub fn parse_stm_list(input: ParseStream) -> syn::Result<Vec<Stmatment>> {
    let res = if input.is_empty() {
        Vec::new()
    } else {

        let stm = input.parse()?;

        let mut res = Vec::new();
        res.push(stm);
        res.append(&mut parse_stm_more(input)?);
        res
    };

    Ok(res)

}

pub fn parse_stm_more(input: ParseStream) -> syn::Result<Vec<Stmatment>> {
    if input.peek(Token![;]) {
        let _semi = input.parse::<Token![;]>()?;
        return parse_stm_list(input);
    }else {
        return Ok(Vec::new());
    }
}


#[derive(Debug)]
pub enum Stmatment {
    Expr(Expr),
    If(IfStm),
    While(WhileStm),
    Write(WriteStm),
    Read(ReadStm),
}

impl ToTokens for Stmatment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Stmatment::Expr(expr) => expr.to_tokens(tokens),
            Stmatment::If(if_stm) => if_stm.to_tokens(tokens),
            Stmatment::While(while_stm) => while_stm.to_tokens(tokens),
            Stmatment::Write(write_stm) => write_stm.to_tokens(tokens),
            Stmatment::Read(read_stm) => read_stm.to_tokens(tokens),
        }
    }
}

pub fn is_peek_kw(input: ParseStream) -> bool {
    let mut res = false;
    res = res
        || input.peek(kw::program)
        || input.peek(kw::begin)
        || input.peek(kw::end)
        || input.peek(kw::procedure)
        || input.peek(kw::r#return)
        || input.peek(kw::r#type)
        || input.peek(kw::r#while)
        || input.peek(kw::r#do)
        || input.peek(kw::endwh)
        || input.peek(kw::r#if)
        || input.peek(kw::then)
        || input.peek(kw::r#else)
        || input.peek(kw::fi)
        || input.peek(kw::char)
        || input.peek(kw::integer)
        || input.peek(kw::record)
        || input.peek(kw::array)
        || input.peek(kw::of)
        || input.peek(kw::read)
        || input.peek(kw::write);

    res
}

fn _is_maybe_end(input: ParseStream) -> bool {
    let mut res = false;
    res = res
        || input.peek(kw::end)
        || input.peek(kw::endwh)
        || input.peek(kw::r#else)
        || input.peek(kw::fi);
    res
}

impl Parse for Stmatment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let res = if lookahead.peek(kw::read) {
            Stmatment::Read(ReadStm::parse(input)?)
        } else if lookahead.peek(kw::write) {
            Stmatment::Write(WriteStm::parse(input)?)
        } else if lookahead.peek(kw::r#if) {
            Stmatment::If(IfStm::parse(input)?)
        } else if lookahead.peek(kw::r#while) {
            Stmatment::While(WhileStm::parse(input)?)
        } else if !is_peek_kw(input) {
            Stmatment::Expr(Expr::parse(input)?)
        } else {
            return Err(lookahead.error());
        };

        //input.parse::<Token![;]>()?;

        Ok(res)
    }
}

// pub fn parse_stm_within(input: ParseStream) -> syn::Result<Vec<Stmatment>> {
//     let mut stmts = Vec::new();
//     loop {        
//         if input.peek(Token![;]) {
//             let _semi = input.parse::<Token![;]>()?;
//         }
//         if input.is_empty() {
//             break;
//         }
//         //println!("{:?}", input);
//         let stm = input.parse()?;
//         //println!("{:?}", input);
//         stmts.push(stm);

//         if input.is_empty() || is_maybe_end(input) {
//             break;
//         }
//     }

//     Ok(stmts)
// }

#[derive(Debug)]
pub struct IfStm {
    pub condition: Expr,
    pub body: Vec<Stmatment>,
    pub else_body: Option<Vec<Stmatment>>,
}

impl ToTokens for IfStm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let condition = self.condition.to_token_stream();

        let body = TokenStream::from_iter(self.body.iter().map(
            |x| {let mut res = x.to_token_stream(); res.extend(quote!{;}); res}));
        if let Some(else_body) = &self.else_body {
            let else_body = TokenStream::from_iter(else_body.iter().map(
                |x| {let mut res = x.to_token_stream(); res.extend(quote!{;}); res}));
            tokens.extend(quote! {
                if #condition {
                    #body
                } else {
                    #else_body
                }
            });
        } else {
            tokens.extend(quote! {
                if #condition {
                    #body
                }
            });
        } 
    }
}

impl Parse for IfStm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::r#if>()?;
        let condition = input.parse()?;
        input.parse::<kw::then>()?;
        let body = parse_stm_list(input)?;
        let else_body = if input.peek(kw::r#else) {
            input.parse::<kw::r#else>()?;
            Some(parse_stm_list(input)?)
        } else {
            None
        };

        input.parse::<kw::fi>()?;

        Ok(IfStm {
            condition,
            body,
            else_body,
        })
    }
}

#[derive(Debug)]
pub struct WhileStm {
    pub condition: Expr,
    pub body: Vec<Stmatment>,
}

impl ToTokens for WhileStm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let condition = self.condition.to_token_stream();
        let body = TokenStream::from_iter(self.body.iter().map(
            |x| {let mut res = x.to_token_stream(); res.extend(quote!{;}); res}));
        tokens.extend(quote! {
            while #condition {
                #body
            }
        });
    }
}

impl Parse for WhileStm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::r#while>()?;
        let condition = input.parse()?;
        input.parse::<kw::r#do>()?;
        let body = parse_stm_list(input)?;
        input.parse::<kw::endwh>()?;

        Ok(WhileStm { condition, body })
    }
}

#[derive(Debug)]
pub struct WriteStm {
    pub param: Expr,
}

impl ToTokens for WriteStm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let param = self.param.to_token_stream();
        tokens.extend(quote! {
            println!("{:?}", #param);
        });
    }
}

impl Parse for WriteStm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::write>()?;
        let content;
        parenthesized!(content in input);
        let param = content.parse()?;

        Ok(WriteStm { param })
    }
}

#[derive(Debug)]
pub struct ReadStm {
    pub param: Expr,
}

impl ToTokens for ReadStm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let param = self.param.to_token_stream();
        tokens.extend(quote! {
            #param = read();
        });
    }
}

impl Parse for ReadStm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::read>()?;
        let content;
        parenthesized!(content in input);
        let param = content.parse()?;

        Ok(ReadStm { param })
    }
}

pub fn snl_stm(input: TokenStream) -> TokenStream {
    let stm: Stmatment = match syn::parse2(input) {
        Ok(stm) => stm,
        Err(e) => {
            return e.to_compile_error();
        }
    };
    println!("{:?}", stm);
    TokenStream::new()
}

pub fn snlc_stm(input: TokenStream) -> TokenStream {
    let stm: Stmatment = match syn::parse2(input) {
        Ok(stm) => stm,
        Err(e) => {
            return e.to_compile_error();
        }
    };
    let res = stm.to_token_stream();
    res
}