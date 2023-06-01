use crate::kw;
use crate::stm::*;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::ParseStream;
use syn::{parenthesized, Token};
use syn::{parse::Parse, punctuated::Punctuated};

fn is_maybe_end(input: ParseStream) -> bool {
    let mut res = false;
    res = res

        || input.peek(kw::begin)
        || input.peek(kw::procedure);
    res
}

fn parse_vardef_within(input: ParseStream) -> syn::Result<Vec<VarDef>> {
    let mut var_defs = Vec::new();

    loop {
        if is_maybe_end(input) || input.is_empty() {
            break;
        }

        let var_def = input.parse()?;
        var_defs.push(var_def);
        let _semi = input.parse::<Token![;]>()?;
    }

    Ok(var_defs)
}

#[derive(Debug)]
pub struct Program {
    pub name: syn::Ident,
    pub var_defs: Option<Vec<VarDef>>,
    pub procedure_defs: Option<Vec<ProcedureDef>>,
    pub body: Vec<Stmatment>,
}

impl ToTokens for Program {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name.to_token_stream();
        let var_defs = if let Some(var_defs) = &self.var_defs {
            let var_defs = TokenStream::from_iter(var_defs.iter().map(|x| x.to_token_stream()));
            quote! {
                #var_defs
            }
        } else {
            quote! {}
        };
        let procedure_defs = if let Some(procedure_defs) = &self.procedure_defs {
            let procedure_defs = TokenStream::from_iter(procedure_defs.iter().map(|x| x.to_token_stream()));
            quote! {
                #procedure_defs
            }
        } else {
            quote! {}
        };

        let body = TokenStream::from_iter(self.body.iter().map(
            |x| {let mut res = x.to_token_stream(); res.extend(quote!{;}); res}));

        tokens.extend(quote! {
            let mut #name = || {
                #var_defs
                #procedure_defs
                #body
            };
            #name();
        })
    }
}

fn parse_procdef_within(input: ParseStream) -> syn::Result<Vec<ProcedureDef>> {
    let mut procs = Vec::new();
    loop {
        if (is_maybe_end(input) || input.is_empty()) && !input.peek(kw::procedure) {
            break;
        }

        let proc = input.parse()?;
        procs.push(proc);
    }
    Ok(procs)
}

impl Parse for Program {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::program>()?;
        let name = input.parse()?;
        let var_defs = if input.peek(kw::var) {
            input.parse::<kw::var>()?;
            Some(parse_vardef_within(input)?)
        } else {
            None
        };

        let procedure_defs = if input.peek(kw::procedure) {
            Some(parse_procdef_within(input)?)
        } else {
            None
        };
        // begin
        input.parse::<kw::begin>()?;
        let body = parse_stm_list(input)?;
        // end
        input.parse::<kw::end>()?;
        // Dot
        input.parse::<Token![.]>()?;

        Ok(Program {
            name,
            var_defs,
            procedure_defs,
            body,
        })
    }
}

#[derive(Debug)]
pub enum VarDef {
    CharTyVar(CharTyVarDef),
    IntTyVar(IntTyVarDef),
}

impl ToTokens for VarDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VarDef::CharTyVar(var_def) => {
                var_def.to_tokens(tokens);
            }
            VarDef::IntTyVar(var_def) => {
                var_def.to_tokens(tokens);
            }
        }
    }
}

impl Parse for VarDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let res = if lookahead.peek(kw::char) {
            VarDef::CharTyVar(CharTyVarDef::parse(input)?)
        } else if lookahead.peek(kw::integer) {
            VarDef::IntTyVar(IntTyVarDef::parse(input)?)
        } else {
            return Err(lookahead.error());
        };

        Ok(res)
    }
}

fn parse_ident_within_vardef(input: ParseStream) -> syn::Result<Vec<syn::Ident>> {
    let mut idents = Vec::new();
    loop {
        if input.peek(Token![;]) || input.is_empty() {
            break;
        }

        let ident = input.parse()?;
        idents.push(ident);

        if !input.peek(Token![;]) {
            let _comma = input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }

    Ok(idents)
}

#[derive(Debug)]
pub struct CharTyVarDef {
    pub ty_kw: kw::char,
    pub names: Vec<syn::Ident>,
}

impl ToTokens for CharTyVarDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ty_kw = &self.ty_kw;
        for name in &self.names {
            tokens.extend(quote! {
                let mut #name = '\0';
            })
        }
    }
}

impl Parse for CharTyVarDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty_kw = input.parse::<kw::char>()?;
        let names = parse_ident_within_vardef(input)?;

        Ok(CharTyVarDef { ty_kw, names })
    }
}

#[derive(Debug)]
pub struct IntTyVarDef {
    pub ty_kw: kw::integer,
    pub names: Vec<syn::Ident>,
}

impl ToTokens for IntTyVarDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ty_kw = &self.ty_kw;
        for name in &self.names {
            tokens.extend(quote! {
                let mut #name = 0;
            })
        }
    }
}

impl Parse for IntTyVarDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty_kw = input.parse::<kw::integer>()?;
        let names = parse_ident_within_vardef(input)?;

        Ok(IntTyVarDef { ty_kw, names })
    }
}


#[derive(Debug)]
pub enum ParamDecl {
    CharTyParam(CharTyParamDecl),
    IntTyParam(IntTyParamDecl),
}

impl ToTokens for ParamDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ParamDecl::CharTyParam(x) => x.to_tokens(tokens),
            ParamDecl::IntTyParam(x) => x.to_tokens(tokens),
        }
    }
}

impl Parse for ParamDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let res = if lookahead.peek(kw::char) {
            ParamDecl::CharTyParam(CharTyParamDecl::parse(input)?)
        } else if lookahead.peek(kw::integer) {
            ParamDecl::IntTyParam(IntTyParamDecl::parse(input)?)
        } else {
            return Err(lookahead.error());
        };

        Ok(res)
    }
}

#[derive(Debug)]
pub struct CharTyParamDecl {
    pub ty_kw: kw::char,
    pub name: syn::Ident,
}

impl ToTokens for CharTyParamDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ty_kw = &self.ty_kw;
        let name = &self.name;
        tokens.extend(quote! {
            #name: char
        })
    }
}

impl Parse for CharTyParamDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty_kw = input.parse()?;
        let name = input.parse()?;

        Ok(CharTyParamDecl { ty_kw, name })
    }
}

#[derive(Debug)]
pub struct IntTyParamDecl {
    pub ty_kw: kw::integer,
    pub name: syn::Ident,
}

impl ToTokens for IntTyParamDecl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ty_kw = &self.ty_kw;
        let name = &self.name;
        tokens.extend(quote! {
            #name: i32
        })
    }
}

impl Parse for IntTyParamDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty_kw = input.parse()?;
        let name = input.parse()?;

        Ok(IntTyParamDecl { ty_kw, name })
    }
}


#[derive(Debug)]
pub struct ProcedureDef {
    pub name: syn::Ident,
    pub params: Punctuated<ParamDecl, Token![,]>,
    pub var_defs: Option<Vec<VarDef>>,
    pub body: Vec<Stmatment>,
}

impl ToTokens for ProcedureDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name.to_token_stream();
        let mut params = quote!{};
        // generate params and append a comma, exclude last param
        let params_len = self.params.len();
        for (i, param) in self.params.iter().enumerate() {
            param.to_tokens(&mut params);
            if i != params_len - 1{
                params.extend(quote!{,});
            }
        }
        let var_defs = match &self.var_defs {
            Some(var_defs) => {
                let mut var_defs_tokens = quote!{};
                for var_def in var_defs {
                    var_def.to_tokens(&mut var_defs_tokens);
                }
                var_defs_tokens
            },
            None => quote!{},
        };
        let mut body = quote!{};
        for stm in &self.body {
            stm.to_tokens(&mut body);
            body.extend(quote!{;});
        }
        tokens.extend(quote! {
            let mut #name = |#params| {
                #var_defs
                #body
            };
        })
    }
}
    

impl Parse for ProcedureDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::procedure>()?;
        let name = input.parse()?;
        let content;
        parenthesized!(content in input);
        let params = Punctuated::parse_terminated(&content)?;
        let var_defs = if input.peek(kw::var) {
            input.parse::<kw::var>()?;
            Some(parse_vardef_within(input)?)
        } else {
            None
        };
        //begin
        input.parse::<kw::begin>()?;
        let body = parse_stm_list(input)?;
        //end    r#array [1..20] r#of r#integer a;

        Ok(ProcedureDef {
            name,
            params,
            var_defs,
            body,
        })
    }
}

pub fn snl_program(input: TokenStream) -> TokenStream {
    let prog: Program = match syn::parse2(input) {
        Ok(prog) => prog,
        Err(e) => return e.to_compile_error(),
    };
    println!("{:?}", prog);
    TokenStream::new()
}

pub fn snlc_program(input: TokenStream) -> TokenStream {
    let prog: Program = match syn::parse2(input) {
        Ok(prog) => prog,
        Err(e) => return e.to_compile_error(),
    };

    prog.to_token_stream()
}
