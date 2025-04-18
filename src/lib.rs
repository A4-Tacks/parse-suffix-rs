#![doc = include_str!("../README.md")]

use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing::*, Span, TokenStream,
    TokenTree,
};

#[must_use]
fn stream<I>(iter: I) -> TokenStream
where I: IntoIterator,
      TokenStream: FromIterator<I::Item>,
{
    TokenStream::from_iter(iter)
}

fn err<T>(msg: &str, span: Span) -> Result<T, TokenStream> {
    let s = |mut t: TokenTree| {
        t.set_span(span);
        t
    };
    Err(stream([
        s(Punct::new(':', Joint).into()),
        s(Punct::new(':', Joint).into()),
        s(Ident::new("core", span).into()),
        s(Punct::new(':', Joint).into()),
        s(Punct::new(':', Joint).into()),
        s(Ident::new("compile_error", span).into()),
        s(Punct::new('!', Joint).into()),
        s(Group::new(Delimiter::Brace, stream([
            s(Literal::string(msg).into()),
        ])).into()),
    ]))
}

fn split_suffix(s: &str, span: Span) -> Result<(&str, &str), TokenStream> {
    if s.starts_with('"') {
        let i = s.rfind('"').unwrap();
        return Ok(s.split_at(i+1));
    }
    if !s.starts_with('r') {
        return err("invalid string literal", span);
    }
    let i = s.rfind(['"', '#']).unwrap();
    Ok(s.split_at(i+1))
}

/// Process the string suffix as `.parse::<suffix>().unwrap()`
///
/// # Examples
/// ```
/// use std::{net::Ipv4Addr, path::PathBuf};
///
/// #[parse_suffix::parse_string_suffix]
/// fn test() {
///     assert_eq!("23"i32, 23);
///     assert_eq!("23"PathBuf, PathBuf::from("23"));
///     assert_eq!("true"bool, true);
///     assert_eq!("false"bool, false);
///     assert_eq!("192.168.1.1"Ipv4Addr, Ipv4Addr::new(192, 168, 1, 1));
///     assert_eq!(r"192.168.1.1"Ipv4Addr, Ipv4Addr::new(192, 168, 1, 1));
///     assert_eq!(r#"192.168.1.1"#Ipv4Addr, Ipv4Addr::new(192, 168, 1, 1));
/// }
/// # test()
/// ```
#[proc_macro_attribute]
pub fn parse_string_suffix(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Some(attr) = attr.into_iter().next() {
        return err::<()>("invalid input", attr.span()).unwrap_err();
    }
    item.into_iter()
        .map(do_token)
        .collect()
}

fn gen_parse(str: &str, suf: &str, span: Span) -> TokenTree {
    let s = |mut tt1: TokenTree| {
        tt1.set_span(span);
        tt1
    };
    s(Group::new(Delimiter::None, stream([
        s(str.parse::<Literal>().unwrap().into()),
        s(Punct::new('.', Joint).into()),
        Ident::new("parse", span).into(),
        s(Punct::new(':', Joint).into()),
        s(Punct::new(':', Joint).into()),
        s(Punct::new('<', Joint).into()),
        Ident::new(suf, span).into(),
        s(Punct::new('>', Joint).into()),
        s(Group::new(Delimiter::Parenthesis, TokenStream::new()).into()),
        s(Punct::new('.', Joint).into()),
        Ident::new("unwrap", span).into(),
        s(Group::new(Delimiter::Parenthesis, TokenStream::new()).into()),
    ])).into())
}

fn do_token(tt: TokenTree) -> TokenTree {
    let s = |mut tt1: TokenTree| {
        tt1.set_span(tt.span());
        tt1
    };
    match tt {
        TokenTree::Group(ref group) => {
            let t = parse_string_suffix(TokenStream::new(), group.stream());
            s(Group::new(group.delimiter(), t).into())
        },
        TokenTree::Ident(_) => tt,
        TokenTree::Punct(_) => tt,
        TokenTree::Literal(ref lit) => {
            match split_suffix(&lit.to_string(), lit.span()) {
                Ok((str, suf)) if !suf.is_empty() => {
                    gen_parse(str, suf, tt.span())
                },
                _ => tt,
            }
        },
    }
}
