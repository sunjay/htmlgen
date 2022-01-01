/// Takes a list of expressions and calls `HtmlChild::from` on each of them
#[macro_export]
macro_rules! ch {
    [$($ch:expr),* $(,)?] => {
        vec![
            $($crate::HtmlChild::from($ch)),*
        ]
    };
}

/// Produces a list of `HtmlAttribute` values from `name="value"` pairs.
#[macro_export]
macro_rules! attrs {
    //TODO: Support attribute names that are not valid identifiers
    ($($name:ident = $value:expr),* $(,)?) => {
        vec![
            $($crate::HtmlAttribute {
                name: stringify!($name).into(),
                value: $value.into(),
            }),*
        ]
    };
}
