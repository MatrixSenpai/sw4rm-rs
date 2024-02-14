use darling::{Result, FromMeta, Error};
use quote::{TokenStreamExt, ToTokens};
use proc_macro2::{TokenStream, Ident, Span, Punct, Spacing, TokenTree};

#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RefType {
    Example,
    Link,
    Parameter,
    PathItem,
    RequestBody,
    Response,
    Schema,
    SecurityScheme,
}

impl FromMeta for RefType {
    fn from_string(value: &str) -> Result<Self> {
        match value {
            "example" => Ok(Self::Example),
            "link" => Ok(Self::Link),
            "parameter" => Ok(Self::Parameter),
            "path_item" => Ok(Self::PathItem),
            "request_body" => Ok(Self::RequestBody),
            "response" => Ok(Self::Response),
            "schema" => Ok(Self::Schema),
            "security_scheme" => Ok(Self::SecurityScheme),

            #[allow(unreachable_patterns)]
            _ => Err(Error::unknown_value(value))
        }
    }
}

impl ToTokens for RefType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self {
            Self::Example => "Example",
            Self::Link => "Link",
            Self::Parameter => "Parameter",
            Self::PathItem => "PathItem",
            Self::RequestBody => "RequestBody",
            Self::Response => "Response",
            Self::Schema => "Schema",
            Self::SecurityScheme => "SecurityScheme",

            #[allow(unreachable_patterns)]
            _ => return
        };

        tokens.append_all(
            vec![
                TokenTree::Ident(Ident::new("resolve_core", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("RefType", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new(ident, Span::call_site())),
            ]
        );
    }
}