use std::borrow::Cow;

use crate::{HtmlElement, ch};

pub use crate::raw_html;

macro_rules! tags {
    ($($el:ident)*) => (
        $(
            pub fn $el<A, C>(attrs: A, children: C) -> $crate::HtmlElement
                where
                    A: Into<Vec<$crate::HtmlAttribute>>,
                    C: Into<Vec<$crate::HtmlChild>>,
            {
                $crate::HtmlElement {
                    name: stringify!($el).into(),
                    attrs: attrs.into(),
                    children: children.into(),
                }
            }
        )*
    );
}

tags! {
    a
    abbr
    address
    area
    article
    aside
    audio
    b
    bdi
    bdo
    blockquote
    body
    br
    button
    canvas
    caption
    cite
    code
    col
    colgroup
    command
    datalist
    dd
    del
    details
    dfn
    div
    dl
    dt
    em
    embed
    fieldset
    figcaption
    figure
    footer
    form
    h1
    h2
    h3
    h4
    h5
    h6
    head
    header
    hr
    html
    i
    iframe
    img
    input
    ins
    kbd
    keygen
    label
    legend
    li
    link
    main
    map
    mark
    menu
    meta
    meter
    nav
    object
    ol
    optgroup
    option
    output
    p
    param
    pre
    progress
    q
    rp
    rt
    ruby
    s
    samp
    script
    section
    select
    small
    source
    span
    strong
    sub
    summary
    sup
    table
    tbody
    td
    textarea
    tfoot
    th
    thead
    time
    tr
    track
    u
    ul
    var
    video
    wbr
}

pub fn title<S>(title: S) -> HtmlElement
where
    S: Into<Cow<'static, str>>,
{
    HtmlElement {
        name: "title".into(),
        attrs: Vec::new(),
        children: ch![
            title.into(),
        ],
    }
}
