use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{
    Attribute, AttrStyle, Expr, ExprLit, Lit, LitStr, MacroDelimiter, Meta, MetaList,
    MetaNameValue, Path, PathArguments, PathSegment, Token,
    __private::Span,
    punctuated::Punctuated,
    token::{
        Bracket, Comma, Paren,
    }
};

pub fn parse_struct_attrs(
    description: Option<String>,
) -> Vec<Attribute> {
    let mut attrs = Vec::new();

    if let Some(description) = create_description_attr(&description) {
        attrs.push(description);
    }

    let mut derive_segments = Punctuated::new();
    derive_segments.push(PathSegment { ident: Ident::new("derive", Span::call_site()), arguments: PathArguments::None });
    let derive_token_stream = quote::quote! {
        Debug, Serialize, Deserialize, Default, Clone, PartialEq
    };
    let derive_meta = MetaList {
        path: Path { leading_colon: None, segments: derive_segments },
        delimiter: MacroDelimiter::Paren(Paren::default()),
        tokens: derive_token_stream,
    };
    let derive_attr = Attribute {
        pound_token: Token![#](Span::call_site()),
        style: AttrStyle::Outer,
        bracket_token: Bracket::default(),
        meta: Meta::List(derive_meta),
    };
    attrs.push(derive_attr);

    let mut default_segments = Punctuated::new();
    default_segments.push(PathSegment { ident: Ident::new("default", Span::call_site()), arguments: PathArguments::None });
    let default_meta = Path {
        leading_colon: None,
        segments: default_segments,
    };

    let mut rename_segments = Punctuated::new();
    rename_segments.push(PathSegment { ident: Ident::new("rename_all", Span::call_site()), arguments: PathArguments::None });
    let rename_expression = ExprLit {
        attrs: Vec::default(),
        lit: Lit::Str(LitStr::new("camelCase", Span::call_site()))
    };
    let rename_meta = MetaNameValue {
        path: Path { leading_colon: None, segments: rename_segments },
        eq_token: Token![=](Span::call_site()),
        value: Expr::Lit(rename_expression)
    };

    let mut serde_attr_segments: Punctuated<TokenStream, Comma> = Punctuated::new();
    serde_attr_segments.push(default_meta.to_token_stream());
    serde_attr_segments.push(rename_meta.to_token_stream());

    let mut serde_segments = Punctuated::new();
    serde_segments.push(PathSegment { ident: Ident::new("serde", Span::call_site()), arguments: PathArguments::None });
    let serde_meta = MetaList {
        path: Path { leading_colon: None, segments: serde_segments },
        delimiter: MacroDelimiter::Paren(Paren::default()),
        tokens: serde_attr_segments.to_token_stream()
    };
    let serde_attr = Attribute {
        pound_token: Token![#](Span::call_site()),
        style: AttrStyle::Outer,
        bracket_token: Bracket::default(),
        meta: Meta::List(serde_meta),
    };

    attrs.push(serde_attr);

    attrs
}

pub fn parse_field_attrs(
    description: &Option<String>,
    field_name: &String,
    needs_rename: bool,
) -> Vec<Attribute> {
    let mut attrs = Vec::new();

    if let Some(description) = create_description_attr(description) {
        attrs.push(description);
    }

    if needs_rename {
        let mut rename_segments = Punctuated::new();
        rename_segments.push(PathSegment { ident: Ident::new("rename", Span::call_site()), arguments: PathArguments::None });
        let rename_expression = ExprLit {
            attrs: Vec::default(),
            lit: Lit::Str(LitStr::new(field_name, Span::call_site()))
        };
        let rename_meta = MetaNameValue {
            path: Path { leading_colon: None, segments: rename_segments },
            eq_token: Token![=](Span::call_site()),
            value: Expr::Lit(rename_expression)
        };
        let mut serde_segments = Punctuated::new();
        serde_segments.push(PathSegment { ident: Ident::new("serde", Span::call_site()), arguments: PathArguments::None });
        let serde_meta = MetaList {
            path: Path { leading_colon: None, segments: serde_segments },
            delimiter: MacroDelimiter::Paren(Paren::default()),
            tokens: rename_meta.to_token_stream()
        };
        let serde_attr = Attribute {
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
            bracket_token: Bracket::default(),
            meta: Meta::List(serde_meta),
        };
        attrs.push(serde_attr);
    }

    attrs
}

fn create_description_attr(
    description: &Option<String>,
) -> Option<Attribute> {
    let description = description.clone()?;
    let description = format!(" {}", description);

    let mut comment_segments = Punctuated::new();
    comment_segments.push(PathSegment {
        ident: Ident::new("doc", Span::call_site()),
        arguments: PathArguments::None
    });

    let comment_expression = ExprLit {
        attrs: Vec::default(),
        lit: Lit::Str(LitStr::new(
            description.as_str(), Span::call_site()
        ))
    };

    let comment_meta = MetaNameValue {
        path: Path { leading_colon: None, segments: comment_segments },
        eq_token: Token![=](Span::call_site()),
        value: Expr::Lit(comment_expression),
    };
    Some(
        Attribute {
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
            bracket_token: Bracket::default(),
            meta: Meta::NameValue(comment_meta)
        }
    )
}