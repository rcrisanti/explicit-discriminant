#![deny(warnings, unsafe_code)]

use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use proc_macro::TokenStream;
use proc_macro_error::{abort, emit_error, emit_warning, proc_macro_error};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, ExprLit, ExprUnary, Lit, Pat, PatParen, RangeLimits, Result, Token, UnOp,
};

/// Require that all variants of an enum have an explicit discriminant defined.
///
/// Example:
/// ```
/// # use explicit_discriminant::ExplicitDiscriminant;
/// // This works
/// #[derive(ExplicitDiscriminant)]
/// pub enum MyEnum {
///     One = 1,
///     Two = 2,
///     Three = 3,
/// }
/// ```
/// ```compule_fail
/// // But this won't compile
/// #[derive(ExplicitDiscriminant)]
/// pub enum MyOtherEnum {
///     One = 1,
///     Two,
///     Three = 3,
/// }
/// ```
#[proc_macro_error]
#[proc_macro_derive(ExplicitDiscriminant, attributes(pattern))]
pub fn derive_explicit_discriminant(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);
    let syn::Data::Enum(data_enum) = input.data else {
        abort!(input, "can only be derived on an enum")
    };

    let punctuated_patterns = match input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("pattern"))
        .map(|pat_attr| {
            pat_attr.parse_args_with(Punctuated::<DisciminantPattern, Token![,]>::parse_terminated)
        })
        .collect::<Result<Vec<_>>>()
    {
        Ok(patterns) => patterns,
        Err(err) => return err.into_compile_error().into(),
    };
    let patterns = punctuated_patterns
        .iter()
        .flat_map(|puncts| puncts.iter().map(|pat| pat.pat.clone()))
        .collect::<Vec<_>>();

    for variant in data_enum.variants {
        if let Some((_, discriminant)) = variant.discriminant {
            if !punctuated_patterns.is_empty()
                && !patterns.iter().any(|pat| tok_matches(&discriminant, pat))
            {
                emit_error!(discriminant, "discriminant does not match any pattern")
            }
        } else {
            emit_error!(variant, "no explicit discriminant")
        }
    }

    TokenStream::new()
}

struct DisciminantPattern {
    pat: Pat,
}

impl Parse for DisciminantPattern {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            pat: Pat::parse_multi(input)?,
        })
    }
}

/// Recreating part of the behavior for `matches!`, but for syn Tokens
fn tok_matches(expr: &Expr, pat: &Pat) -> bool {
    let expr_int = expr_as_int(expr);

    match pat {
        Pat::Lit(exprlit) => expr_int == exprlit_as_int(exprlit),
        Pat::Or(pator) => pator.cases.iter().any(|case| tok_matches(expr, case)),
        Pat::Range(patrange) => {
            let start = patrange.start.as_ref().map(|expr| expr_as_int(expr));
            let end = patrange.end.as_ref().map(|expr| expr_as_int(expr));
            match (start, patrange.limits, end) {
                (Some(start), RangeLimits::Closed(_), Some(end)) => {
                    RangeInclusive::new(start, end).contains(&expr_int)
                }
                (Some(start), RangeLimits::HalfOpen(_), Some(end)) => {
                    Range { start, end }.contains(&expr_int)
                }
                (Some(start), RangeLimits::HalfOpen(_), None) => {
                    RangeFrom { start }.contains(&expr_int)
                }
                (None, RangeLimits::Closed(_), Some(end)) => {
                    RangeToInclusive { end }.contains(&expr_int)
                }
                (None, RangeLimits::HalfOpen(_), Some(end)) => RangeTo { end }.contains(&expr_int),
                _ => abort!(patrange, "unsupported range type"),
            }
        }
        Pat::Wild(_) => true,
        Pat::Paren(PatParen { pat, .. }) => tok_matches(expr, pat),
        _ => {
            emit_warning!(
                pat,
                "Currently supported are: literals, ranges, or-patterns, parenthesizeds, and wilds"
            );
            abort!(pat, format!("pattern type not supported"));
        }
    }
}

fn expr_as_int(expr: &Expr) -> i128 {
    match expr {
        Expr::Lit(lit) => exprlit_as_int(lit),
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr,
            ..
        }) => -expr_as_int(expr),
        _ => abort!(
            expr,
            "only literal expressions (optionally negated) are supported"
        ),
    }
}

fn exprlit_as_int(exprlit: &ExprLit) -> i128 {
    match &exprlit.lit {
        Lit::Int(litint) => litint
            .base10_parse()
            .unwrap_or_else(|_| abort!(litint, "could not parse token to i128")),
        _ => abort!(exprlit.lit, "only integer literals are supported"),
    }
}
