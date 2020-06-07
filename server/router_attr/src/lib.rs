extern crate proc_macro;

use proc_macro2::{Delimiter::Brace, Group, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse::{Nothing, Parse},
    parse2, parse_macro_input, Attribute, ItemFn,
};

struct Branch {
    attr: TokenStream,
    other_attrs: Vec<Attribute>,
    value: syn::Expr,
}

impl ToTokens for Branch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.other_attrs
            .iter()
            .for_each(|attr| attr.to_tokens(tokens));
        self.attr.to_tokens(tokens);
        syn::Token!(=>)(Span::call_site()).to_tokens(tokens);
        TokenTree::Group(Group::new(Brace, self.value.clone().into_token_stream()))
            .to_tokens(tokens);
    }
}

impl Parse for Branch {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr = input.call(syn::Attribute::parse_outer)?;
        let (attr, rest): (Vec<_>, Vec<_>) = attr
            .into_iter()
            .partition(|item| item.path.to_token_stream().to_string() == "route");
        if let Some((first, others)) = attr.split_first() {
            if others.is_empty() {
                match first.tokens.clone().into_iter().next() {
                    Some(TokenTree::Group(group)) => Ok(Branch {
                        attr: group.stream(),
                        other_attrs: rest,
                        value: input.parse()?,
                    }),
                    _ => Err(syn::Error::new_spanned(
                        &first.tokens,
                        "[route] attribute parameters must be grouped by parenthesis",
                    )),
                }
            } else {
                Err(syn::Error::new_spanned(
                    &others[0],
                    "Unexpected attribute, every statement must map to exactly one route",
                ))
            }
        } else {
            Err(syn::Error::new(
                input.span(),
                "You need to add at least one [route] attribute to this statement",
            ))
        }
    }
}

#[proc_macro_attribute]
pub fn router(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let _ = parse_macro_input!(attr as Nothing);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);

    let branches: Vec<Branch> = match block
        .stmts
        .into_iter()
        .map(|item| {
            if let syn::Stmt::Semi(expr, _) = item {
                // Implicitly convert to Branch by serializing and parsing again, so that we don't have to
                // iterate over every possible expression
                parse2(expr.to_token_stream())
            } else {
                Err(syn::Error::new_spanned(
                    item,
                    "Unexpected item, expected semicolon-ending statement",
                ))
            }
        })
        .collect::<Result<_, _>>()
    {
        Ok(vec) => vec,
        Err(error) => return error.to_compile_error().into(),
    };

    let output = quote! {
        #(#attrs)* #vis #sig {
            move |request| ::router_attr_wrapper::rouille::router! {
                request,
                #(#branches),*
            }
        }
    };
    output.into()
}
