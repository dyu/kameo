use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    DeriveInput, Expr, ExprAssign, ExprLit, Ident, Lit, LitStr,
};

pub struct DeriveRemoteActor {
    attrs: DeriveRemoteActorAttrs,
    ident: Ident,
}

impl ToTokens for DeriveRemoteActor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { attrs, ident } = self;

        let id = match &attrs.id {
            Some(id) => quote! { #id },
            None => quote! {
                ::std::concat!(::std::module_path!(), "::", ::std::stringify!(#ident))
            },
        };

        tokens.extend(quote! {
            #[automatically_derived]
            impl ::kameo::remote::RemoteActor for #ident {
                const REMOTE_ID: &'static str = #id;
            }
        });
    }
}

impl Parse for DeriveRemoteActor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input: DeriveInput = input.parse()?;
        let mut attrs = None;
        for attr in input.attrs {
            if attr.path().is_ident("remote_actor") {
                if attrs.is_some() {
                    return Err(syn::Error::new(
                        attr.span(),
                        "remote_actor attribute already specified",
                    ));
                }
                attrs = Some(attr.parse_args_with(DeriveRemoteActorAttrs::parse)?);
            }
        }
        let ident = input.ident;

        Ok(DeriveRemoteActor {
            attrs: attrs.unwrap_or_default(),
            ident,
        })
    }
}

#[derive(Default)]
struct DeriveRemoteActorAttrs {
    id: Option<LitStr>,
}

impl Parse for DeriveRemoteActorAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr: ExprAssign = input
            .parse()
            .map_err(|_| syn::Error::new(input.span(), "expected id = \"...\" expression"))?;
        let Expr::Path(left_path) = expr.left.as_ref() else {
            return Err(syn::Error::new(expr.left.span(), "expected `id`"));
        };
        if !left_path.path.is_ident("id") {
            return Err(syn::Error::new(expr.left.span(), "expected `id`"));
        }
        let Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }) = *expr.right
        else {
            return Err(syn::Error::new(
                expr.right.span(),
                "expected a string literal",
            ));
        };

        Ok(DeriveRemoteActorAttrs { id: Some(lit_str) })
    }
}