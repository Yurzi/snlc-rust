use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{bracketed, parenthesized, Token};
use syn::{parse::Parse, parse::ParseStream};

#[derive(Debug)]
pub enum Expr {
    Assign(ExprAssign),
    Binary(ExprBinary),
    Var(ExprVar),
    Lit(ExprLit),
    Index(ExprIndex),
    Call(ExprCall),
    Paren(ExprParen),
}

impl ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Expr::Assign(expr) => expr.to_tokens(tokens),
            Expr::Binary(expr) => expr.to_tokens(tokens),
            Expr::Var(expr) => expr.to_tokens(tokens),
            Expr::Lit(expr) => expr.to_tokens(tokens),
            Expr::Index(expr) => expr.to_tokens(tokens),
            Expr::Call(expr) => expr.to_tokens(tokens),
            Expr::Paren(expr) => expr.to_tokens(tokens),
        }
    }
}

#[derive(Debug)]
pub enum BinOp {
    Lt,
    Le,
    Eq,

    Assign,

    Plus,
    Minus,
    Star,
    Slash,
    Unknown,
}
impl ToTokens for BinOp {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            BinOp::Lt => tokens.append(Punct::new('<', Spacing::Alone)),
            BinOp::Le => {
                tokens.append(Punct::new('<', Spacing::Joint));
                tokens.append(Punct::new('=', Spacing::Alone));
            }
            BinOp::Eq => {
                tokens.append(Punct::new('=', Spacing::Joint));
                tokens.append(Punct::new('=', Spacing::Alone));
            }
            BinOp::Assign => {
                tokens.append(Punct::new('=', Spacing::Alone));
            }
            BinOp::Plus => {
                tokens.append(Punct::new('+', Spacing::Alone));
            }
            BinOp::Minus => {
                tokens.append(Punct::new('-', Spacing::Alone));
            }
            BinOp::Star => {
                tokens.append(Punct::new('*', Spacing::Alone));
            }
            BinOp::Slash => tokens.append(Punct::new('/', Spacing::Alone)),
            _ => (),
        };
    }
}
impl Parse for BinOp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ahead = input.fork();
        let lookahead = input.lookahead1();
        let res: BinOp = if lookahead.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            BinOp::Plus
        } else if lookahead.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            BinOp::Minus
        } else if lookahead.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            BinOp::Star
        } else if lookahead.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            BinOp::Slash
        } else if lookahead.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                BinOp::Le
            } else {
                BinOp::Lt
            }
        } else if lookahead.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            BinOp::Eq
        } else if lookahead.peek(Token![:]) {
            ahead.parse::<Token![:]>()?;
            let ahead_lookahead = ahead.lookahead1();
            if ahead_lookahead.peek(Token![=]) {
                input.parse::<Token![:]>()?;
                input.parse::<Token![=]>()?;
                BinOp::Assign
            } else {
                return Err(ahead_lookahead.error());
            }
        } else {
            return Err(lookahead.error());
        };

        Ok(res)
    }
}

#[derive(Debug)]
pub enum Lit {
    Char(syn::LitChar),
    Integer(syn::LitInt),
}

#[derive(Debug)]
pub struct ExprAssign {
    pub target: Box<Expr>,
    pub from: Box<Expr>,
}

impl ToTokens for ExprAssign {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let target = self.target.to_token_stream();
        let op = BinOp::Assign.to_token_stream();
        let from = self.from.to_token_stream();
        tokens.extend(quote! {
            #target #op #from
        });
        }
}

#[derive(Debug)]
pub struct ExprBinary {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
}

impl ToTokens for ExprBinary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lhs = self.lhs.to_token_stream();
        let op = self.op.to_token_stream();
        let rhs = self.rhs.to_token_stream();
        tokens.extend(quote! {
            #lhs #op #rhs
        });
    }
}

#[derive(Debug)]
pub struct ExprVar {
    pub ident: syn::Ident,
}

impl ToTokens for ExprVar {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident.to_token_stream();
        tokens.extend(quote! {
            #ident
        });
    }
}

#[derive(Debug)]
pub struct ExprLit {
    pub lit: Lit,
}
impl ToTokens for ExprLit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lit = match &self.lit {
            Lit::Char(lit) => lit.to_token_stream(),
            Lit::Integer(lit) => lit.to_token_stream(),
        };
        tokens.extend(quote! {
            #lit
        });
    }
}

#[derive(Debug)]
pub struct ExprIndex {
    pub ident: syn::Ident,
    pub bracket_token: syn::token::Bracket,
    pub index: Box<Expr>,
}

impl ToTokens for ExprIndex {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident.to_token_stream();
        let index = self.index.to_token_stream();
        tokens.extend(quote! {
            #ident[#index]
        });
    }
}

#[derive(Debug)]
pub struct ExprCall {
    pub ident: syn::Ident,
    pub paren_token: syn::token::Paren,
    pub args: Vec<Expr>,
}

impl ToTokens for ExprCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident.to_token_stream();
        let args = &self.args;
        let mut args_tokenstream = quote!{};
        let args_len = args.len();
        for (i, arg) in args.iter().enumerate() {
            let arg_tokens = arg.to_token_stream();
            if i == args_len - 1 {
                args_tokenstream.extend(quote! {
                    #arg_tokens
                });
            } else {
                args_tokenstream.extend(quote! {
                    #arg_tokens,
                });
            }
        }
        tokens.extend(quote! {
            #ident(#args_tokenstream)
        });
    }
}

#[derive(Debug)]
pub struct ExprParen {
    pub paren_token: syn::token::Paren,
    pub expr: Box<Expr>,
}

impl ToTokens for ExprParen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = self.expr.to_token_stream();
        tokens.extend(quote! {
            (#expr)
        });
    }
}

fn expr_tailer(input: ParseStream) -> syn::Result<(BinOp, Expr)> {
    let lookahead = input.lookahead1();
    if lookahead.peek(Token![;]) {
        return Err(lookahead.error());
    };

    let op = input.parse()?;
    let expr = input.parse()?;

    Ok((op, expr))
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ahead = input.fork();
        let lookahead = input.lookahead1();
        let lhs: Expr = if lookahead.peek(syn::Lit) {
            Expr::Lit(ExprLit::parse(input)?)
        } else if lookahead.peek(syn::Ident) {
            ahead.parse::<syn::Ident>()?;
            if ahead.peek(syn::token::Bracket) {
                Expr::Index(ExprIndex::parse(input)?)
            } else if ahead.peek(syn::token::Paren) {
                Expr::Call(ExprCall::parse(input)?)
            } else {
                Expr::Var(ExprVar::parse(input)?)
            }
        } else if lookahead.peek(syn::token::Paren) {
            Expr::Paren(ExprParen::parse(input)?)
        } else {
            return Err(lookahead.error());
        };

        let (op, rhs) = match expr_tailer(input) {
            Ok((op, rhs)) => (op, rhs),
            Err(_) => {
                return Ok(lhs);
            }
        };

        let res = match op {
            BinOp::Assign => Expr::Assign(ExprAssign {
                target: Box::new(lhs),
                from: Box::new(rhs),
            }),
            _ => Expr::Binary(ExprBinary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }),
        };
        Ok(res)
    }
}

impl Parse for ExprAssign {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target = input.parse()?;
        let _op: BinOp = input.parse()?;
        let from = input.parse()?;

        Ok(ExprAssign { target, from })
    }
}

impl Parse for ExprBinary {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lhs = input.parse()?;
        let op = input.parse()?;
        let rhs = input.parse()?;

        Ok(ExprBinary { lhs, op, rhs })
    }
}

impl Parse for ExprVar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        Ok(ExprVar { ident })
    }
}

impl Parse for ExprLit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let lit = if lookahead.peek(syn::LitChar) {
            Lit::Char(input.parse::<syn::LitChar>()?)
        } else if lookahead.peek(syn::LitInt) {
            Lit::Integer(input.parse::<syn::LitInt>()?)
        } else {
            return Err(lookahead.error());
        };

        Ok(ExprLit { lit })
    }
}

impl Parse for ExprIndex {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let content;
        let bracket_token = bracketed!(content in input);
        let index = content.parse::<Expr>()?;

        Ok(ExprIndex {
            ident,
            bracket_token,
            index: Box::new(index),
        })
    }
}

impl Parse for ExprCall {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let content;
        let paren_token = parenthesized!(content in input);
        let mut args = Vec::new();
        while !content.is_empty() {
            let expr = content.parse::<Expr>()?;
            args.push(expr);
            if content.is_empty() {
                break;
            }
            content.parse::<Token![,]>()?;
        }
        Ok(ExprCall {
            ident,
            paren_token,
            args,
        })
    }
}

impl Parse for ExprParen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let expr = content.parse()?;

        Ok(ExprParen { paren_token, expr })
    }
}

pub fn snl_expr(input: TokenStream) -> TokenStream {
    let expr: Expr = match syn::parse2(input) {
        Ok(expr) => expr,
        Err(e) => return e.to_compile_error(),
    };
    println!("{:?}", expr);

    TokenStream::new()
}

pub fn snlc_expr(input: TokenStream) -> TokenStream {
    let expr: Expr = match syn::parse2(input) {
        Ok(expr) => expr,
        Err(e) => return e.to_compile_error(),
    };

    let mut expr_stream = quote!{};

    expr_stream.extend(expr.to_token_stream());

    expr_stream
} 