use std::fmt;
use std::borrow::Cow;

use html_escape::{encode_safe, encode_double_quoted_attribute};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HtmlElement {
    /// The tag name of this HTML element
    pub name: Cow<'static, str>,
    /// The HTML attributes of this element
    pub attrs: Vec<HtmlAttribute>,
    /// The child elements or text objects under this element
    pub children: Vec<HtmlChild>,
}

impl fmt::Display for HtmlElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {name, attrs, children} = self;

        //TODO: `script`, `style`, and maybe some other elements need special handling.
        //  The `html-escape` crate provides functions for most of these cases.

        match &**name {
            "html" => write!(f, "<!DOCTYPE html>")?,
            _ => {},
        }

        write!(f, "<{}", name)?;
        for attr in attrs {
            write!(f, " {}", attr)?;
        }
        write!(f, ">")?;

        for child in children {
            write!(f, "{}", child)?;
        }
        if !children.is_empty() {
            write!(f, "</{}>", name)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HtmlAttribute {
    /// The name of the attribute
    pub name: Cow<'static, str>,
    /// The value of the attribute
    pub value: Cow<'static, str>,
}

impl fmt::Display for HtmlAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {name, value} = self;

        // Simple validation based on: https://stackoverflow.com/a/926136/551904
        for ch in name.chars() {
            match ch {
                '\t' | '\n' | '\u{000C}' | ' ' | '\\' | '/' | '>' | '"' | '\'' | '=' => {
                    panic!("invalid HTML attribute name: `{}`", name);
                },
                _ => {},
            }
        }

        write!(f, "{}=\"{}\"", name, encode_double_quoted_attribute(value))
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum HtmlChild {
    Element(HtmlElement),
    /// Text data that will be HTML escaped
    Text(Cow<'static, str>),
    /// Raw HTML data that will not be HTML escaped
    RawHtml(Cow<'static, str>),
}

impl From<HtmlElement> for HtmlChild {
    fn from(el: HtmlElement) -> Self {
        Self::Element(el)
    }
}

impl<S> From<S> for HtmlChild
where
    S: Into<Cow<'static, str>>,
{
    fn from(text: S) -> Self {
        Self::Text(text.into())
    }
}

/// Returns an HTML child that will be rendered as is, without escaping any HTML in its contents
pub fn raw_html<S: Into<Cow<'static, str>>>(s: S) -> HtmlChild {
    HtmlChild::RawHtml(s.into())
}

impl fmt::Display for HtmlChild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HtmlChild::*;
        match self {
            Element(el) => write!(f, "{}", el),
            Text(text) => write!(f, "{}", encode_safe(text)),
            RawHtml(html) => write!(f, "{}", html),
        }
    }
}
