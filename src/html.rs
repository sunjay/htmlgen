use std::borrow::Cow;

use crate::Array;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HtmlElement {
    /// The tag name of this HTML element
    pub name: Cow<'static, str>,
    /// The HTML attributes of this element
    pub attrs: Cow<'static, [HtmlAttribute]>,
    /// The child elements or text objects under this element
    pub children: Array<'static, HtmlChild>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HtmlAttribute {
    //TODO
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum HtmlChild {
    Element(HtmlElement),
    Text(Cow<'static, str>),
}
