use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HtmlElement {
    /// The tag name of this HTML element
    pub name: Cow<'static, str>,
    /// The HTML attributes of this element
    pub attrs: Cow<'static, [HtmlAttribute]>,
    /// The child elements or text objects under this element
    pub children: Vec<HtmlChild>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HtmlAttribute {
    /// The name of the attribute
    pub name: Cow<'static, str>,
    /// The value of the attribute
    pub value: Cow<'static, str>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum HtmlChild {
    Element(HtmlElement),
    Text(Cow<'static, str>),
}

impl From<HtmlElement> for HtmlChild {
    fn from(el: HtmlElement) -> Self {
        Self::Element(el)
    }
}

impl From<Cow<'static, str>> for HtmlChild {
    fn from(text: Cow<'static, str>) -> Self {
        Self::Text(text)
    }
}
